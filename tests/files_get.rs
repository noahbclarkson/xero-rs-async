// tests/files_get.rs

mod common;

#[tokio::test]
async fn get_folders_and_by_id() {
    let test_client = common::get_test_client().await;
    let api = test_client.client.files_for_tenant(test_client.tenant_id);

    let result = api.get_folders(None).await;
    let folders = result.expect("API call to get folders failed");

    assert!(
        folders.iter().any(|f| f.name == "Inbox"),
        "Expected to find the default 'Inbox' folder."
    );
    println!("Successfully retrieved {} folders.", folders.len());

    // Test getting a single folder by ID
    let inbox_folder = folders
        .iter()
        .find(|f| f.name == "Inbox")
        .expect("Inbox folder not found");
    let inbox_id = inbox_folder.id;

    let single_result = api.get_folder_by_id(inbox_id).await;
    let single_folder = single_result.expect("Failed to get single folder by ID");

    assert_eq!(
        single_folder.id, inbox_id,
        "Returned folder ID does not match requested ID."
    );
    assert_eq!(single_folder.name, "Inbox");
    println!(
        "Successfully retrieved single folder by ID: {}",
        single_folder.name
    );
}

#[tokio::test]
async fn get_files_and_content() {
    let test_client = common::get_test_client().await;
    let api = test_client.client.files_for_tenant(test_client.tenant_id);

    let result = api.get_files(None, None, None, None).await;
    let files = result.expect("API call to get files failed");

    println!("Successfully retrieved {} files.", files.len());

    // If there are files, test getting one by ID and its content
    if let Some(first_file) = files.first() {
        let file_id = first_file.id;

        // Get file by ID
        let single_result = api.get_file_by_id(file_id).await;
        let single_file = single_result.expect("Failed to get single file by ID");
        assert_eq!(single_file.id, file_id);
        println!(
            "Successfully retrieved single file by ID: {}",
            single_file.name
        );

        // Get file content
        let content_result = api.get_file_content(file_id).await;
        let content = content_result.expect("Failed to get file content");
        assert_eq!(content.len() as u64, single_file.size);
        println!(
            "Successfully retrieved file content for '{}' ({} bytes).",
            single_file.name,
            content.len()
        );
    } else {
        println!("Skipping get_file_by_id and get_file_content tests as no files were found.");
    }
}

#[tokio::test]
async fn get_associations() {
    let test_client = common::get_test_client().await;
    let files_api = test_client.client.files_for_tenant(test_client.tenant_id);
    let accounting_api = test_client
        .client
        .accounting_for_tenant(test_client.tenant_id);

    // We need an object ID to test with. Let's get an invoice.
    let invoices = accounting_api
        .get_invoices(
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            Some(1),
            None,
            None,
            None,
        )
        .await
        .expect("Failed to get invoices to test associations");

    if let Some(invoice) = invoices.first() {
        let object_id = invoice.invoice_id.unwrap();

        // Test getting associations for that object
        let object_assoc_result = files_api.get_object_associations(object_id).await;
        let object_associations = object_assoc_result.expect("Failed to get object associations");
        println!(
            "Found {} associations for invoice {}",
            object_associations.len(),
            object_id
        );

        // Test getting association count
        let count_result = files_api.get_associations_count(vec![object_id]).await;
        let count_map = count_result.expect("Failed to get association count");
        let count = count_map.get(&object_id).unwrap_or(&0);
        assert_eq!(*count as usize, object_associations.len());
        println!("Association count for invoice {object_id} is {count}.");

        // If there's an association, test getting it by file ID
        if let Some(association) = object_associations.first() {
            let file_id = association.file_id;
            let file_assoc_result = files_api.get_file_associations(file_id).await;
            let file_associations = file_assoc_result.expect("Failed to get file associations");
            assert!(!file_associations.is_empty());
            println!(
                "Found {} associations for file {}",
                file_associations.len(),
                file_id
            );
        } else {
            println!(
                "Skipping get_file_associations test as no associations were found on the invoice."
            );
        }
    } else {
        println!("Skipping association tests as no invoices were found.");
    }
}
