use anyhow::Ok;
use common::utils::calculate_request_id;
use near_workspaces::types::NearToken;
use serde_json::json;

pub mod common;

const EARTHMIND_WASM_FILEPATH : &str= "/Users/hasselalcala/Documents/near_contracts/machilabs_earthmind/earthmind-rs/target/wasm32-unknown-unknown/release/earthmind_rs.wasm";

#[tokio::test]
async fn test_earthmind_contract() -> anyhow::Result<()> {
    // @dev Setting up sandbox and deploy the earthmind contract
    let worker = near_workspaces::sandbox().await?;
    let wasm = std::fs::read(EARTHMIND_WASM_FILEPATH)?;

    let contract = worker.dev_deploy(&wasm).await?;

    let outcome = contract.call("new").transact().await?;

    println!("new_contract outcome: {:#?}", outcome);
    assert!(outcome.is_success());

    //@dev Register a protocol
    let protocol_account = worker.dev_create_account().await?;
    println!("protocol acount created: {:?}", protocol_account);

    let registration_fee = NearToken::from_near(10);

    let outcome = protocol_account
                                           .call(contract.id(), "register_protocol")
                                           .args_json(json!({"culture":"Governance decision", "modules":["TextPrompting", "ObjectRecognition"]}))
                                           .deposit(registration_fee)
                                           .transact()
                                           .await?;

    println!("register protocol outcome: {:#?}", outcome);

    let expected = format!(
        r#"EVENT_JSON:{{"standard":"emip001","version":"1.0.0","event":"register_protocol","data":[{{"account":"{}"}}]}}"#,
        protocol_account.id()
    );
    let logs = outcome.logs().join("\n");

    assert_eq!(expected, logs);

    // @dev verify that protocol es registered
    let account = protocol_account.id();
    let result = contract
        .call("is_protocol_registered")
        .args_json(json!({"account":account}))
        .transact()
        .await?;

    println!("is_protocol_registered outcome: {:#?}", result);
    assert!(result.is_success());
    Ok(())
}

#[tokio::test]
async fn test_register_miner() -> anyhow::Result<()> {
    let worker = near_workspaces::sandbox().await?;
    let wasm = std::fs::read(EARTHMIND_WASM_FILEPATH)?;

    let contract = worker.dev_deploy(&wasm).await?;
    let contract_deploy_outcome = contract.call("new").transact().await?;

    assert!(contract_deploy_outcome.is_success());

    // @dev register miner to earthmind protocol
    let miner_account = worker.dev_create_account().await?;
    let miner_deposit = NearToken::from_near(1);

    let register_miner_outcome = miner_account
        .call(contract.id(), "register_miner")
        .deposit(miner_deposit)
        .transact()
        .await?;

    println!("register_miner_outcome: {:#?}", register_miner_outcome);
    assert!(register_miner_outcome.is_success());

    let expected = format!(
        r#"EVENT_JSON:{{"standard":"emip001","version":"1.0.0","event":"register_miner","data":[{{"miner":"{}"}}]}}"#,
        miner_account.id()
    );
    let logs = register_miner_outcome.logs().join("\n");

    assert_eq!(expected, logs);

    // @dev verify miner is registered
    let miner_account = miner_account.id();
    let is_miner_registered_outcome = contract
        .call("is_miner_registered")
        .args_json(json!({"miner_id":miner_account}))
        .transact()
        .await?;

    //TODO: the conection was success but which value obtain?
    // status: SuccessValue('true'),
    assert!(is_miner_registered_outcome.is_success());

    Ok(())
}

#[tokio::test]
async fn test_request_governance_decision() -> anyhow::Result<()> {
    let worker = near_workspaces::sandbox().await?;
    let wasm = std::fs::read(EARTHMIND_WASM_FILEPATH)?;

    let contract = worker.dev_deploy(&wasm).await?;

    let _contract_deploy_outcome = contract.call("new").transact().await?;

    let protocol_account = worker.dev_create_account().await?;
    let registration_fee = NearToken::from_near(10);

    let _register_protocol_outcome = protocol_account
                                           .call(contract.id(), "register_protocol")
                                           .args_json(json!({"culture":"Governance decision", "modules":["TextPrompting", "ObjectRecognition"]}))
                                           .deposit(registration_fee)
                                           .transact()
                                           .await?;

    let request_governance_decision_outcome = protocol_account
        .call(contract.id(), "request_governance_decision")
        .args_json(json!({"message":"Should we change the rules?"}))
        .transact()
        .await?;

    println!(
        "request_governance_decision_outcome: {:#?}",
        request_governance_decision_outcome
    );
    assert!(request_governance_decision_outcome.is_success());

    let request_id = calculate_request_id(
        protocol_account.id().clone(),
        "Should we change the rules?".to_string(),
    );
    let expected = format!(
        r#"EVENT_JSON:{{"standard":"emip001","version":"1.0.0","event":"register_request","data":[{{"request_id":"{}"}}]}}"#,
        request_id
    );
    let logs = request_governance_decision_outcome.logs().join("\n");
    assert_eq!(expected, logs);

    Ok(())
}

/*
#[tokio::test]
async fn test_client_listens_and_responds() {
    let (worker, contract) = init().await;

    // Crear una cuenta para el cliente
    let client_account = worker.root_account().create_subaccount("client").initial_balance(10_000_000_000_000_000_000_000_000).transact().await.unwrap().into_result().unwrap();

    // Crear y configurar el cliente
    let client = Arc::new(Client::new(client_account.id().clone(), client_account.signer().clone(), &worker.rpc_url(), contract.id().clone()));

    // Ejecutar la lógica del cliente en un thread separado
    let client_clone = client.clone();
    let client_handle = tokio::spawn(async move {
        client_clone.listen_and_respond().await;
    });

    // Simular el evento de governance request
    contract.call(&worker, "emit_governance_request")
        .args_json(serde_json::json!({"message": "Governance request"}))
        .transact()
        .await
        .unwrap();

    // Esperar un tiempo para permitir que el cliente procese el evento
    tokio::time::sleep(std::time::Duration::from_secs(10)).await;

    // Verificar los resultados esperados (por ejemplo, un commit exitoso)
    // Aquí necesitarás una forma de verificar que el commit fue realizado
    // Esto podría implicar revisar el estado del contrato o los logs

    // Detener el cliente
    client_handle.abort();
}
 */
