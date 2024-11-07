use crate::{mock::*, Error, Event, Config};
use frame_support::{assert_noop, assert_ok, BoundedVec};
use sp_runtime::traits::{Hash, Get};

/// Helper function to create a bounded vector from a regular vector
/// Used throughout tests to simplify vector creation
fn create_bounded_vec<T: Clone + std::fmt::Debug, S: Get<u32>>(v: Vec<T>) -> BoundedVec<T, S> {
    BoundedVec::try_from(v).unwrap()
}

/// Tests successful storage of weight data
#[test]
fn store_weight_data_works() {
    new_test_ext().execute_with(|| {
        // Set block number for event emission
        System::set_block_number(1);

        // Create test data
        let vector_data = create_bounded_vec::<u8, <Test as Config>::MaxVectorLength>(
            vec![1, 2, 3, 4, 5]
        );
        let tags = vec![vec![1, 2, 3], vec![4, 5, 6]];
        let weight_data = (vector_data, 1, BoundedVec::default());
        let vector_id = <Test as frame_system::Config>::Hashing::hash_of(&weight_data);

        // Store the weight data
        assert_ok!(TemplateModule::store_weight_data(
            RuntimeOrigin::signed(1),
            weight_data,
            tags
        ));

        // Verify event was emitted
        System::assert_has_event(RuntimeEvent::TemplateModule(Event::VectorStored {
            vector_id,
            author: 1,
        }));
    });
}

/// Tests failure when trying to store vector exceeding maximum length
#[test]
fn store_weight_data_fails_with_long_vector() {
    new_test_ext().execute_with(|| {
        // First get the max length
        let max_len: u32 = <Test as Config>::MaxVectorLength::get();
        
        // Create vector that exceeds maximum length
        let long_vector: Vec<u8> = vec![0; (max_len + 1) as usize];
        
        // Try to convert to BoundedVec and verify it fails
        let result: Result<BoundedVec<u8, <Test as Config>::MaxVectorLength>, _> = BoundedVec::try_from(long_vector);
        assert!(result.is_err(), "Vector should be too long");
    });
}

/// Tests successful retrieval of weights by tag
#[test]
fn get_weights_by_tag_works() {
    new_test_ext().execute_with(|| {
        // Store a vector with tags first
        let vector_data = create_bounded_vec::<u8, <Test as Config>::MaxVectorLength>(
            vec![1, 2, 3]
        );
        let tag = vec![1, 2, 3];
        let weight_data = (vector_data, 1, BoundedVec::default());

        assert_ok!(TemplateModule::store_weight_data(
            RuntimeOrigin::signed(1),
            weight_data,
            vec![tag.clone()]
        ));

        // Query by tag
        assert_ok!(TemplateModule::get_weights_by_tag(
            RuntimeOrigin::signed(1),
            tag
        ));
    });
}

/// Tests failure when searching for non-existent tag
#[test]
fn get_weights_by_tag_fails_for_nonexistent_tag() {
    new_test_ext().execute_with(|| {
        assert_noop!(
            TemplateModule::get_weights_by_tag(
                RuntimeOrigin::signed(1),
                vec![1, 2, 3]
            ),
            Error::<Test>::TagNotFound
        );
    });
}

/// Tests successful pruning of weight data
#[test]
fn prune_weight_data_works() {
    new_test_ext().execute_with(|| {
        // Set block number for event emission
        System::set_block_number(1);

        // Store a vector first
        let vector_data = create_bounded_vec::<u8, <Test as Config>::MaxVectorLength>(
            vec![1, 2, 3]
        );
        let weight_data = (vector_data, 1, BoundedVec::default());
        
        assert_ok!(TemplateModule::store_weight_data(
            RuntimeOrigin::signed(1),
            weight_data.clone(),
            vec![]
        ));

        let vector_id = <Test as frame_system::Config>::Hashing::hash_of(&weight_data);

        // Prune the vector
        assert_ok!(TemplateModule::prune_weight_data(
            RuntimeOrigin::signed(1),
            vec![vector_id]
        ));

        // Verify event was emitted
        System::assert_has_event(RuntimeEvent::TemplateModule(Event::VectorsPruned {
            count: 1,
        }));
    });
}

/// Tests failure when trying to prune with empty list
#[test]
fn prune_weight_data_fails_with_empty_list() {
    new_test_ext().execute_with(|| {
        assert_noop!(
            TemplateModule::prune_weight_data(
                RuntimeOrigin::signed(1),
                vec![]
            ),
            Error::<Test>::NothingToPrune
        );
    });
}

/// Tests successful retrieval of weights by author
#[test]
fn get_weights_by_author_works() {
    new_test_ext().execute_with(|| {
        // Store a vector first
        let vector_data = create_bounded_vec::<u8, <Test as Config>::MaxVectorLength>(
            vec![1, 2, 3]
        );
        let weight_data = (vector_data, 1, BoundedVec::default());

        assert_ok!(TemplateModule::store_weight_data(
            RuntimeOrigin::signed(1),
            weight_data,
            vec![]
        ));

        // Query by author
        assert_ok!(TemplateModule::get_weights_by_author(
            RuntimeOrigin::signed(1),
            1
        ));
    });
}
