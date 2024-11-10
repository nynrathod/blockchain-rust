use std::time::{SystemTime, UNIX_EPOCH};

use blocks::{
    AppBlock, AppChangeDataPayload, AppChangeEvent, AppCreateDataPayload, AppCreateEvent,
    AppDataEvent, AppEventType, AppPayload,
};

mod blocks;
mod hashable;
mod library;

fn main() {
    let genesis_payload = AppPayload {
        user_id: 1,
        data: AppDataEvent::Create(AppCreateEvent {
            event_type: "Genesis".to_string(),
            timestamp: get_timestamp(),
            data: AppCreateDataPayload {
                hash: "genesis_hash".to_string(),
                app_name: "My App".to_string(),
                email: "admin@example.com".to_string(),
                password: "password123".to_string(),
                notes: "Genesis Block".to_string(),
            },
        }),
    };

    let genesis_block = AppBlock::new("prev_hash".to_string(), vec![genesis_payload]);

    // Create a second block after the genesis
    let second_payload = AppPayload {
        user_id: 2,
        data: AppDataEvent::Create(AppCreateEvent {
            event_type: "Second Block".to_string(),
            timestamp: get_timestamp(),
            data: AppCreateDataPayload {
                hash: "second_block_hash".to_string(),
                app_name: "My App".to_string(),
                email: "user@example.com".to_string(),
                password: "user_password".to_string(),
                notes: "Second Block".to_string(),
            },
        }),
    };

    let mut second_block = AppBlock::new(genesis_block.hash.clone(), vec![second_payload]);
    second_block.mine_block(&genesis_block, 0);

    // Create a third block with an AppChangeEvent to utilize the Change variant
    let third_payload = AppPayload {
        user_id: 3,
        data: AppDataEvent::Change(AppChangeEvent {
            event_type: AppEventType::AppNameChange, // Using the Change variant
            timestamp: get_timestamp(),
            old_data: AppChangeDataPayload {
                data_hash: "previous_app_name_hash".to_string(),
                data: "Old App Name".to_string(),
            },
            new_data: AppChangeDataPayload {
                data_hash: "new_app_name_hash".to_string(),
                data: "New App Name".to_string(),
            },
        }),
    };

    let mut third_block = AppBlock::new(second_block.hash.clone(), vec![third_payload]);
    third_block.mine_block(&second_block, 0);

    // Create a fourth block with a UsernameChange event
    let fourth_payload = AppPayload {
        user_id: 4,
        data: AppDataEvent::Change(AppChangeEvent {
            event_type: AppEventType::UsernameChange, // Using UsernameChange variant
            timestamp: get_timestamp(),
            old_data: AppChangeDataPayload {
                data_hash: "previous_username_hash".to_string(),
                data: "Old Username".to_string(),
            },
            new_data: AppChangeDataPayload {
                data_hash: "new_username_hash".to_string(),
                data: "New Username".to_string(),
            },
        }),
    };

    let mut fourth_block = AppBlock::new(third_block.hash.clone(), vec![fourth_payload]);
    fourth_block.mine_block(&third_block, 0);

    // Print the blocks
    println!("{:?}", genesis_block);
    println!("{:?}", second_block);
    println!("{:?}", third_block);
    println!("{:?}", fourth_block);
}

fn get_timestamp() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs() as i64
}
