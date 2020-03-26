#![cfg(test)]

pub use super::*;

use sp_core::H256;
use frame_support::{impl_outer_origin, assert_ok, assert_noop, assert_err, parameter_types,
                    weights::Weight, dispatch::DispatchResult};
use sp_runtime::{
  traits::{BlakeTwo256, IdentityLookup}, testing::Header, Perbill,
};

impl_outer_origin! {
  pub enum Origin for Test {}
}

// For testing the module, we construct most of a mock runtime. This means
// first constructing a configuration type (`Test`) which `impl`s each of the
// configuration traits of modules we want to use.
#[derive(Clone, Eq, PartialEq)]
pub struct Test;
parameter_types! {
  pub const BlockHashCount: u64 = 250;
  pub const MaximumBlockWeight: Weight = 1024;
  pub const MaximumBlockLength: u32 = 2 * 1024;
  pub const AvailableBlockRatio: Perbill = Perbill::from_percent(75);
}
impl system::Trait for Test {
  type Origin = Origin;
  type Call = ();
  type Index = u64;
  type BlockNumber = u64;
  type Hash = H256;
  type Hashing = BlakeTwo256;
  type AccountId = u64;
  type Lookup = IdentityLookup<Self::AccountId>;
  type Header = Header;
  type Event = ();
  type BlockHashCount = BlockHashCount;
  type MaximumBlockWeight = MaximumBlockWeight;
  type MaximumBlockLength = MaximumBlockLength;
  type AvailableBlockRatio = AvailableBlockRatio;
  type Version = ();
  type ModuleToIndex = ();
}

parameter_types! {
  pub const MinimumPeriod: u64 = 5;
}
impl pallet_timestamp::Trait for Test {
  type Moment = u64;
  type OnTimestampSet = ();
  type MinimumPeriod = MinimumPeriod;
}

impl Trait for Test {
  type Event = ();
}

type Social = Module<Test>;

// This function basically just builds a genesis storage key/value store according to
// our desired mockup.
fn new_test_ext() -> sp_io::TestExternalities {
  system::GenesisConfig::default().build_storage::<Test>().unwrap().into()
}

pub type AccountId = u64;

const ACCOUNT1 : AccountId = 1;
const ACCOUNT2 : AccountId = 2;

fn blog_handle() -> Vec<u8> {
  b"blog_handle".to_vec()
}

fn blog_ipfs_hash() -> Vec<u8> {
  b"QmRAQB6YaCyidP37UdDnjFY5vQuiBrcqdyoW1CuDgwxkD4".to_vec()
}

fn blog_update(writers: Option<Vec<AccountId>>, handle: Option<Option<Vec<u8>>>, ipfs_hash: Option<Vec<u8>>) -> BlogUpdate<u64> {
  BlogUpdate {
    writers,
    handle,
    ipfs_hash
  }
}

fn post_ipfs_hash() -> Vec<u8> {
  b"QmRAQB6YaCyidP37UdDnjFY5vQuiBrcqdyoW2CuDgwxkD4".to_vec()
}

fn fake_post(id: PostId, created_by: AccountId, blog_id: Option<BlogId>, extension: PostExtension) -> Post<Test> {
  Post {
    id,
    created: WhoAndWhen::<Test>::new(created_by),
    updated: None,
    blog_id,
    extension,
    ipfs_hash: self::post_ipfs_hash(),
    edit_history: vec![],
    total_replies_count: 0,
    shares_count: 0,
    upvotes_count: 0,
    downvotes_count: 0,
    score: 0,
  }
}

fn post_update(blog_id: Option<BlogId>, ipfs_hash: Option<Vec<u8>>) -> PostUpdate {
  PostUpdate {
    blog_id,
    ipfs_hash
  }
}

fn comment_ipfs_hash() -> Vec<u8> {
  b"QmRAQB6YaCyidP37UdDnjFY5vQuiBrcqdyoW1CuDgwxkD4".to_vec()
}

fn subcomment_ipfs_hash() -> Vec<u8> {
  b"QmYA2fn8cMbVWo4v95RwcwJVyQsNtnEwHerfWR8UNtEwoE".to_vec()
}

fn alice_username() -> Vec<u8> {
  b"Alice".to_vec()
}
fn bob_username() -> Vec<u8> {
  b"Bob".to_vec()
}

fn profile_ipfs_hash() -> Vec<u8> {
  b"QmRAQB6YaCyidP37UdDnjFY5vQuiaRtqdyoW2CuDgwxkA5".to_vec()
}

fn reaction_upvote() -> ReactionKind {
  ReactionKind::Upvote
}
fn reaction_downvote() -> ReactionKind {
  ReactionKind::Downvote
}

fn scoring_action_upvote_post() -> ScoringAction {
  ScoringAction::UpvotePost
}
fn scoring_action_downvote_post() -> ScoringAction {
  ScoringAction::DownvotePost
}
fn scoring_action_share_post() -> ScoringAction {
  ScoringAction::SharePost
}
fn scoring_action_create_comment() -> ScoringAction {
  ScoringAction::CreateComment
}
fn scoring_action_upvote_comment() -> ScoringAction {
  ScoringAction::UpvoteComment
}
fn scoring_action_downvote_comment() -> ScoringAction {
  ScoringAction::DownvoteComment
}
fn scoring_action_share_comment() -> ScoringAction {
  ScoringAction::ShareComment
}
fn scoring_action_follow_blog() -> ScoringAction {
  ScoringAction::FollowBlog
}
fn scoring_action_follow_account() -> ScoringAction {
  ScoringAction::FollowAccount
}

fn extension_regular_post() -> PostExtension {
  PostExtension::RegularPost
}
fn extension_comment(parent_id: Option<PostId>, root_post_id: PostId) -> PostExtension {
  PostExtension::Comment(CommentExt{ parent_id, root_post_id })
}
fn extension_shared_post(post_id: PostId) -> PostExtension {
  PostExtension::SharedPost(post_id)
}

fn _create_default_blog() -> DispatchResult {
  _create_blog(None, None, None)
}

fn _create_blog(origin: Option<Origin>, handle: Option<Option<Vec<u8>>>, ipfs_hash: Option<Vec<u8>>) -> DispatchResult {
  Social::create_blog(
    origin.unwrap_or(Origin::signed(ACCOUNT1)),
    handle.unwrap_or(Some(self::blog_handle())),
    ipfs_hash.unwrap_or(self::blog_ipfs_hash())
  )
}

fn _update_blog(origin: Option<Origin>, blog_id: Option<u32>, update: Option<BlogUpdate<u64>>) -> DispatchResult {
  Social::update_blog(
    origin.unwrap_or(Origin::signed(ACCOUNT1)),
    blog_id.unwrap_or(1).into(),
    update.unwrap_or(self::blog_update(None, None, None))
  )
}

fn _default_follow_blog() -> DispatchResult {
  _follow_blog(None, None)
}

fn _follow_blog(origin: Option<Origin>, blog_id: Option<BlogId>) -> DispatchResult {
  Social::follow_blog(
    origin.unwrap_or(Origin::signed(ACCOUNT2)),
    blog_id.unwrap_or(1)
  )
}

fn _default_unfollow_blog() -> DispatchResult {
  _unfollow_blog(None, None)
}

fn _unfollow_blog(origin: Option<Origin>, blog_id: Option<BlogId>) -> DispatchResult {
  Social::unfollow_blog(
    origin.unwrap_or(Origin::signed(ACCOUNT2)),
    blog_id.unwrap_or(1)
  )
}

fn _create_default_post() -> DispatchResult {
  _create_post(None, None, None, None)
}

fn _create_post(origin: Option<Origin>, blog_id_opt: Option<Option<BlogId>>, extension: Option<PostExtension>, ipfs_hash: Option<Vec<u8>>) -> DispatchResult {
  Social::create_post(
    origin.unwrap_or(Origin::signed(ACCOUNT1)),
    blog_id_opt.unwrap_or(Some(1)),
    extension.unwrap_or(self::extension_regular_post()),
    ipfs_hash.unwrap_or(self::post_ipfs_hash())
  )
}

fn _update_post(origin: Option<Origin>, post_id: Option<PostId>, update: Option<PostUpdate>) -> DispatchResult {
  Social::update_post(
    origin.unwrap_or(Origin::signed(ACCOUNT1)),
    post_id.unwrap_or(1),
    update.unwrap_or(self::post_update(None, None))
  )
}

fn _create_default_comment() -> DispatchResult {
  _create_comment(None, None, None, None)
}

fn _create_comment(origin: Option<Origin>, post_id: Option<PostId>,
                   parent_id: Option<Option<PostId>>, ipfs_hash: Option<Vec<u8>>)
  -> DispatchResult {
  _create_post(
    origin,
    Some(None),
    Some(self::extension_comment(
      parent_id.unwrap_or(None), post_id.unwrap_or(1))
    ),
    Some(ipfs_hash.unwrap_or(self::comment_ipfs_hash()))
  )
}

fn _update_comment(origin: Option<Origin>, post_id: Option<PostId>, update: Option<PostUpdate>) -> DispatchResult {
  _update_post(
    origin,
    Some(post_id.unwrap_or(2)),
    Some(update.unwrap_or(
      self::post_update(None, Some(self::subcomment_ipfs_hash())))
    )
  )
}

fn _create_default_post_reaction() -> DispatchResult {
  _create_post_reaction(None, None, None)
}

fn _create_default_comment_reaction() -> DispatchResult {
  _create_comment_reaction(None, None, None)
}

fn _create_post_reaction(origin: Option<Origin>, post_id: Option<PostId>, kind: Option<ReactionKind>) -> DispatchResult {
  Social::create_post_reaction(
    origin.unwrap_or(Origin::signed(ACCOUNT1)),
    post_id.unwrap_or(1),
    kind.unwrap_or(self::reaction_upvote())
  )
}

fn _create_comment_reaction(origin: Option<Origin>, post_id: Option<PostId>, kind: Option<ReactionKind>) -> DispatchResult {
  _create_post_reaction(origin, Some(post_id.unwrap_or(2)), kind)
}

fn _update_post_reaction(origin: Option<Origin>, post_id: Option<PostId>, reaction_id: ReactionId, kind: Option<ReactionKind>) -> DispatchResult {
  Social::update_post_reaction(
    origin.unwrap_or(Origin::signed(ACCOUNT1)),
    post_id.unwrap_or(1),
    reaction_id,
    kind.unwrap_or(self::reaction_upvote())
  )
}

fn _update_comment_reaction(origin: Option<Origin>, post_id: Option<PostId>, reaction_id: ReactionId, kind: Option<ReactionKind>) -> DispatchResult {
  _update_post_reaction(origin, Some(post_id.unwrap_or(2)), reaction_id, kind)
}

fn _delete_post_reaction(origin: Option<Origin>, post_id: Option<PostId>, reaction_id: ReactionId) -> DispatchResult {
  Social::delete_post_reaction(
    origin.unwrap_or(Origin::signed(ACCOUNT1)),
    post_id.unwrap_or(1),
    reaction_id
  )
}

fn _delete_comment_reaction(origin: Option<Origin>, post_id: Option<PostId>, reaction_id: ReactionId) -> DispatchResult {
  _delete_post_reaction(origin, Some(post_id.unwrap_or(2)), reaction_id)
}

fn _create_default_profile() -> DispatchResult {
  _create_profile(None, None, None)
}

fn _create_profile(origin: Option<Origin>, username: Option<Vec<u8>>, ipfs_hash: Option<Vec<u8>>) -> DispatchResult {
  Social::create_profile(
    origin.unwrap_or(Origin::signed(ACCOUNT1)),
    username.unwrap_or(self::alice_username()),
    ipfs_hash.unwrap_or(self::profile_ipfs_hash())
  )
}

fn _update_profile(origin: Option<Origin>, username: Option<Vec<u8>>, ipfs_hash: Option<Vec<u8>>) -> DispatchResult {
  Social::update_profile(
    origin.unwrap_or(Origin::signed(ACCOUNT1)),
    ProfileUpdate {
      username,
      ipfs_hash
    }
  )
}

fn _default_follow_account() -> DispatchResult {
  _follow_account(None, None)
}

fn _follow_account(origin: Option<Origin>, account: Option<AccountId>) -> DispatchResult {
  Social::follow_account(
    origin.unwrap_or(Origin::signed(ACCOUNT2)),
    account.unwrap_or(ACCOUNT1)
  )
}

fn _default_unfollow_account() -> DispatchResult {
  _unfollow_account(None, None)
}

fn _unfollow_account(origin: Option<Origin>, account: Option<AccountId>) -> DispatchResult {
  Social::unfollow_account(
    origin.unwrap_or(Origin::signed(ACCOUNT2)),
    account.unwrap_or(ACCOUNT1)
  )
}

fn _change_post_score_by_extension_with_id(account: AccountId, post_id: PostId, action: ScoringAction) -> DispatchResult {
  if let Some(ref mut post) = Social::post_by_id(post_id) {
    Social::change_post_score_by_extension(account, post, action)
  } else {
    panic!("Test error. Post\\Comment with specified ID not found.");
  }
}

fn _change_post_score_by_extension(account: AccountId, post: &mut Post<Test>, action: ScoringAction) -> DispatchResult {
  Social::change_post_score_by_extension(account, post, action)
}

// Blog tests
#[test]
fn create_blog_should_work() {
  new_test_ext().execute_with(|| {
    assert_ok!(_create_default_blog()); // BlogId 1

    // Check storages
    assert_eq!(Social::blog_ids_by_owner(ACCOUNT1), vec![1]);
    assert_eq!(Social::blog_id_by_handle(self::blog_handle()), Some(1));
    assert_eq!(Social::next_blog_id(), 2);

    // Check whether data stored correctly
    let blog = Social::blog_by_id(1).unwrap();

    assert_eq!(blog.created.account, ACCOUNT1);
    assert_eq!(blog.handle, Some(self::blog_handle()));
    assert_eq!(blog.ipfs_hash, self::blog_ipfs_hash());
    assert!(blog.writers.is_empty());
    assert_eq!(blog.posts_count, 0);
    assert_eq!(blog.followers_count, 1);
    assert!(blog.edit_history.is_empty());
  });
}

#[test]
fn create_blog_should_make_handle_lowercase() {
  let handle : Vec<u8> = b"bLoG_hAnDlE".to_vec();

  new_test_ext().execute_with(|| {
    assert_ok!(_create_blog(None, Some(Some(handle.clone())), None)); // BlodId 1

    let blog = Social::blog_by_id(1).unwrap();
    assert_eq!(blog.handle, Some(handle.to_ascii_lowercase()));
  });
}

#[test]
fn create_blog_should_fail_short_handle() {
  let handle : Vec<u8> = vec![65; (DEFAULT_HANDLE_MIN_LEN - 1) as usize];

  new_test_ext().execute_with(|| {
    // Try to catch an error creating a blog with too short handle
    assert_err!(_create_blog(None, Some(Some(handle)), None), Error::<Test>::HandleIsTooShort);
  });
}

#[test]
fn create_blog_should_fail_long_handle() {
  let handle : Vec<u8> = vec![65; (DEFAULT_HANDLE_MAX_LEN + 1) as usize];

  new_test_ext().execute_with(|| {
    // Try to catch an error creating a blog with too long handle
    assert_err!(_create_blog(None, Some(Some(handle)), None), Error::<Test>::HandleIsTooLong);
  });
}

#[test]
fn create_blog_should_fail_not_unique_handle() {

  new_test_ext().execute_with(|| {
    assert_ok!(_create_default_blog()); // BlogId 1
    // Try to catch an error creating a blog with not unique handle
    assert_err!(_create_default_blog(), Error::<Test>::HandleIsNotUnique);
  });
}

#[test]
fn create_blog_should_fail_invalid_at_char() {
  let handle : Vec<u8> = b"@blog_handle".to_vec();

  new_test_ext().execute_with(|| {
    assert_err!(_create_blog(None, Some(Some(handle.clone())), None), Error::<Test>::HandleContainsInvalidChars);
  });
}

#[test]
fn create_blog_should_fail_invalid_minus_char() {
  let handle : Vec<u8> = b"blog-handle".to_vec();

  new_test_ext().execute_with(|| {
    assert_err!(_create_blog(None, Some(Some(handle.clone())), None), Error::<Test>::HandleContainsInvalidChars);
  });
}

#[test]
fn create_blog_should_fail_invalid_space_char() {
  let handle : Vec<u8> = b"blog handle".to_vec();

  new_test_ext().execute_with(|| {
    assert_err!(_create_blog(None, Some(Some(handle.clone())), None), Error::<Test>::HandleContainsInvalidChars);
  });
}

#[test]
fn create_blog_should_fail_invalid_unicode_char() {
  let handle : Vec<u8> = String::from("блог_хендл").into_bytes().to_vec();

  new_test_ext().execute_with(|| {
    assert_err!(_create_blog(None, Some(Some(handle.clone())), None), Error::<Test>::HandleContainsInvalidChars);
  });
}

#[test]
fn create_blog_should_fail_invalid_ipfs_hash() {
  let ipfs_hash : Vec<u8> = b"QmV9tSDx9UiPeWExXEeH6aoDvmihvx6j".to_vec();

  new_test_ext().execute_with(|| {
    // Try to catch an error creating a blog with invalid ipfs_hash
    assert_noop!(_create_blog(None, None, Some(ipfs_hash)), Error::<Test>::IpfsIsIncorrect);
  });
}

#[test]
fn update_blog_should_work() {
  let handle : Vec<u8> = b"new_handle".to_vec();
  let ipfs_hash : Vec<u8> = b"QmRAQB6YaCyidP37UdDnjFY5vQuiBrcqdyoW2CuDgwxkD4".to_vec();

  new_test_ext().execute_with(|| {
    assert_ok!(_create_default_blog()); // BlogId 1

    // Blog update with ID 1 should be fine
    assert_ok!(_update_blog(None, None,
      Some(
        self::blog_update(
          None,
          Some(Some(handle.clone())),
          Some(ipfs_hash.clone())
        )
      )
    ));

    // Check whether blog updates correctly
    let blog = Social::blog_by_id(1).unwrap();
    assert_eq!(blog.handle, Some(handle));
    assert_eq!(blog.ipfs_hash, ipfs_hash);

    // Check whether history recorded correctly
    assert_eq!(blog.edit_history[0].old_data.writers, None);
    assert_eq!(blog.edit_history[0].old_data.handle, Some(Some(self::blog_handle())));
    assert_eq!(blog.edit_history[0].old_data.ipfs_hash, Some(self::blog_ipfs_hash()));
  });
}

#[test]
fn update_blog_should_fail_nothing_to_update() {
  new_test_ext().execute_with(|| {
    assert_ok!(_create_default_blog()); // BlogId 1

    // Try to catch an error updating a blog with no changes
    assert_noop!(_update_blog(None, None, None), Error::<Test>::NoUpdatesInBlog);
  });
}

#[test]
fn update_blog_should_fail_blog_not_found() {
  let handle : Vec<u8> = b"new_handle".to_vec();

  new_test_ext().execute_with(|| {
    assert_ok!(_create_default_blog()); // BlogId 1

    // Try to catch an error updating a blog with wrong blog ID
    assert_noop!(_update_blog(None, Some(2),
      Some(
        self::blog_update(
          None,
          Some(Some(handle)),
          None
        )
      )
    ), Error::<Test>::BlogNotFound);
  });
}

#[test]
fn update_blog_should_fail_not_an_owner() {
  let handle : Vec<u8> = b"new_handle".to_vec();

  new_test_ext().execute_with(|| {
    assert_ok!(_create_default_blog()); // BlogId 1

    // Try to catch an error updating a blog with different account
    assert_noop!(_update_blog(Some(Origin::signed(ACCOUNT2)), None,
      Some(
        self::blog_update(
          None,
          Some(Some(handle)),
          None
        )
      )
    ), Error::<Test>::NotABlogOwner);
  });
}

#[test]
fn update_blog_should_fail_short_handle() {
  let handle : Vec<u8> = vec![65; (DEFAULT_HANDLE_MIN_LEN - 1) as usize];

  new_test_ext().execute_with(|| {
    assert_ok!(_create_default_blog()); // BlogId 1

    // Try to catch an error updating a blog with too short handle
    assert_err!(_update_blog(None, None,
      Some(
        self::blog_update(
          None,
          Some(Some(handle)),
          None
        )
      )
    ), Error::<Test>::HandleIsTooShort);
  });
}

#[test]
fn update_blog_should_fail_long_handle() {
  let handle : Vec<u8> = vec![65; (DEFAULT_HANDLE_MAX_LEN + 1) as usize];

  new_test_ext().execute_with(|| {
    assert_ok!(_create_default_blog()); // BlogId 1

    // Try to catch an error updating a blog with too long handle
    assert_err!(_update_blog(None, None,
      Some(
        self::blog_update(
          None,
          Some(Some(handle)),
          None
        )
      )
    ), Error::<Test>::HandleIsTooLong);
  });
}

#[test]
fn update_blog_should_fail_not_unique_handle() {
  let handle : Vec<u8> = b"unique_handle".to_vec();

  new_test_ext().execute_with(|| {
    assert_ok!(_create_default_blog()); // BlogId 1

    assert_ok!(_create_blog(
      None,
      Some(Some(handle.clone())),
      None
    )); // BlogId 2 with a custom handle

    // Try to catch an error updating a blog on ID 1 with a handle of blog on ID 2
    assert_err!(_update_blog(None, Some(1),
      Some(
        self::blog_update(
          None,
          Some(Some(handle)),
          None
        )
      )
    ), Error::<Test>::HandleIsNotUnique);
  });
}

#[test]
fn update_blog_should_fail_invalid_at_char() {
  let handle : Vec<u8> = b"@blog_handle".to_vec();

  new_test_ext().execute_with(|| {
    assert_ok!(_create_default_blog()); // BlogId 1

    assert_err!(_update_blog(None, None,
      Some(
        self::blog_update(
          None,
          Some(Some(handle)),
          None
        )
      )
    ), Error::<Test>::HandleContainsInvalidChars);
  });
}

#[test]
fn update_blog_should_fail_invalid_minus_char() {
  let handle : Vec<u8> = b"blog-handle".to_vec();

  new_test_ext().execute_with(|| {
    assert_ok!(_create_default_blog()); // BlogId 1

    assert_err!(_update_blog(None, None,
      Some(
        self::blog_update(
          None,
          Some(Some(handle)),
          None
        )
      )
    ), Error::<Test>::HandleContainsInvalidChars);
  });
}

#[test]
fn update_blog_should_fail_invalid_space_char() {
  let handle : Vec<u8> = b"blog handle".to_vec();

  new_test_ext().execute_with(|| {
    assert_ok!(_create_default_blog()); // BlogId 1

    assert_err!(_update_blog(None, None,
      Some(
        self::blog_update(
          None,
          Some(Some(handle)),
          None
        )
      )
    ), Error::<Test>::HandleContainsInvalidChars);
  });
}

#[test]
fn update_blog_should_fail_invalid_unicode_char() {
  let handle : Vec<u8> = String::from("блог_хендл").into_bytes().to_vec();

  new_test_ext().execute_with(|| {
    assert_ok!(_create_default_blog()); // BlogId 1

    assert_err!(_update_blog(None, None,
      Some(
        self::blog_update(
          None,
          Some(Some(handle)),
          None
        )
      )
    ), Error::<Test>::HandleContainsInvalidChars);
  });
}

#[test]
fn update_blog_should_fail_invalid_ipfs_hash() {
  let ipfs_hash : Vec<u8> = b"QmV9tSDx9UiPeWExXEeH6aoDvmihvx6j".to_vec();

  new_test_ext().execute_with(|| {
    assert_ok!(_create_default_blog()); // BlogId 1

    // Try to catch an error updating a blog with invalid ipfs_hash
    assert_noop!(_update_blog(None, None,
      Some(
        self::blog_update(
          None,
          None,
          Some(ipfs_hash)
        )
      )
    ), Error::<Test>::IpfsIsIncorrect);
  });
}

// Post tests
#[test]
fn create_post_should_work() {
  new_test_ext().execute_with(|| {
    assert_ok!(_create_default_blog()); // BlogId 1
    assert_ok!(_create_default_post()); // PostId 1

    // Check storages
    assert_eq!(Social::post_ids_by_blog_id(1), vec![1]);
    assert_eq!(Social::next_post_id(), 2);

    // Check whether data stored correctly
    let post = Social::post_by_id(1).unwrap();

    assert_eq!(post.created.account, ACCOUNT1);
    assert!(post.updated.is_none());

    assert_eq!(post.blog_id, Some(1));
    assert_eq!(post.extension, self::extension_regular_post());

    assert_eq!(post.ipfs_hash, self::post_ipfs_hash());
    assert!(post.edit_history.is_empty());

    assert_eq!(post.total_replies_count, 0);
    assert_eq!(post.shares_count, 0);
    assert_eq!(post.upvotes_count, 0);
    assert_eq!(post.downvotes_count, 0);

    assert_eq!(post.score, 0);
  });
}

#[test]
fn create_post_should_fail_blog_not_found() {
  new_test_ext().execute_with(|| {
    assert_noop!(_create_default_post(), Error::<Test>::BlogNotFound);
  });
}

#[test]
fn create_post_should_fail_invalid_ipfs_hash() {
  let ipfs_hash : Vec<u8> = b"QmV9tSDx9UiPeWExXEeH6aoDvmihvx6j".to_vec();

  new_test_ext().execute_with(|| {
    assert_ok!(_create_default_blog()); // BlogId 1

    // Try to catch an error creating a regular post with invalid ipfs_hash
    assert_noop!(_create_post(None, None, None, Some(ipfs_hash)), Error::<Test>::IpfsIsIncorrect);
  });
}

#[test]
fn update_post_should_work() {
  let ipfs_hash : Vec<u8> = b"QmRAQB6YaCyidP37UdDnjFY5vQuiBrcqdyoW1CuDgwxkD4".to_vec();

  new_test_ext().execute_with(|| {
    assert_ok!(_create_default_blog()); // BlogId 1
    assert_ok!(_create_default_post()); // PostId 1

    // Post update with ID 1 should be fine
    assert_ok!(_update_post(None, None,
      Some(
        self::post_update(
          None,
          Some(ipfs_hash.clone())
        )
      )
    ));


    // Check whether post updates correctly
    let post = Social::post_by_id(1).unwrap();
    assert_eq!(post.blog_id, Some(1));
    assert_eq!(post.ipfs_hash, ipfs_hash);

    // Check whether history recorded correctly
    assert_eq!(post.edit_history[0].old_data.blog_id, None);
    assert_eq!(post.edit_history[0].old_data.ipfs_hash, Some(self::post_ipfs_hash()));
  });
}

#[test]
fn update_post_should_fail_nothing_to_update() {
  new_test_ext().execute_with(|| {
    assert_ok!(_create_default_blog()); // BlogId 1
    assert_ok!(_create_default_post()); // PostId 1

    // Try to catch an error updating a post with no changes
    assert_noop!(_update_post(None, None, None), Error::<Test>::NoUpdatesInPost);
  });
}

#[test]
fn update_post_should_fail_post_not_found() {
  new_test_ext().execute_with(|| {
    assert_ok!(_create_default_blog()); // BlogId 1
    assert_ok!(_create_blog(None, Some(Some(b"blog2_handle".to_vec())), None)); // BlogId 2
    assert_ok!(_create_default_post()); // PostId 1

    // Try to catch an error updating a post with wrong post ID
    assert_noop!(_update_post(None, Some(2),
      Some(
        self::post_update(
          Some(2),
          None
        )
      )
    ), Error::<Test>::PostNotFound);
  });
}

#[test]
fn update_post_should_fail_not_an_owner() {
  new_test_ext().execute_with(|| {
    assert_ok!(_create_default_blog()); // BlogId 1
    assert_ok!(_create_blog(None, Some(Some(b"blog2_handle".to_vec())), None)); // BlogId 2
    assert_ok!(_create_default_post()); // PostId 1

    // Try to catch an error updating a post with different account
    assert_noop!(_update_post(Some(Origin::signed(ACCOUNT2)), None,
      Some(
        self::post_update(
          Some(2),
          None
        )
      )
    ), Error::<Test>::NotAnAuthor);
  });
}

#[test]
fn update_post_should_fail_invalid_ipfs_hash() {
  let ipfs_hash : Vec<u8> = b"QmV9tSDx9UiPeWExXEeH6aoDvmihvx6j".to_vec();

  new_test_ext().execute_with(|| {
    assert_ok!(_create_default_blog()); // BlogId 1
    assert_ok!(_create_default_post()); // PostId 1

    // Try to catch an error updating a post with invalid ipfs_hash
    assert_noop!(_update_post(None, None,
      Some(
        self::post_update(
          None,
          Some(ipfs_hash)
        )
      )
    ), Error::<Test>::IpfsIsIncorrect);
  });
}

// Comment tests
#[test]
fn create_comment_should_work() {
  new_test_ext().execute_with(|| {
    assert_ok!(_create_default_blog()); // BlogId 1
    assert_ok!(_create_default_post()); // PostId 1
    assert_ok!(_create_default_comment()); // PostId 2

    // Check storages
    assert_eq!(Social::comment_ids_by_post_id(1), vec![2]);
    assert_eq!(Social::post_by_id(1).unwrap().total_replies_count, 1);

    // Check whether data stored correctly
    let comment = Social::post_by_id(2).unwrap();
    let comment_ext = comment.get_comment_ext().unwrap();

    assert_eq!(comment_ext.parent_id, None);
    assert_eq!(comment_ext.root_post_id, 1);
    assert_eq!(comment.created.account, ACCOUNT1);
    assert!(comment.updated.is_none());
    assert_eq!(comment.ipfs_hash, self::comment_ipfs_hash());
    assert!(comment.edit_history.is_empty());
    assert_eq!(comment.total_replies_count, 0);
    assert_eq!(comment.shares_count, 0);
    assert_eq!(comment.upvotes_count, 0);
    assert_eq!(comment.downvotes_count, 0);
    assert_eq!(comment.score, 0);
  });
}

#[test]
fn create_comment_should_work_with_parent() {
  new_test_ext().execute_with(|| {
    assert_ok!(_create_default_blog()); // BlogId 1
    assert_ok!(_create_default_post()); // PostId 1
    assert_ok!(_create_default_comment()); // PostId 2
    assert_ok!(_create_comment(None, None, Some(Some(2)), None)); // PostId 3 with parent comment with PostId 2

    // Check storages
    assert_eq!(Social::comment_ids_by_post_id(1), vec![2, 3]);
    assert_eq!(Social::post_by_id(1).unwrap().total_replies_count, 2);

    // Check whether data stored correctly
    let comment_ext = Social::post_by_id(3).unwrap().get_comment_ext().unwrap();
    assert_eq!(comment_ext.parent_id, Some(2));
  });
}

#[test]
fn create_comment_should_fail_post_not_found() {
  new_test_ext().execute_with(|| {
    // Try to catch an error creating a comment with wrong post
    assert_noop!(_create_default_comment(), Error::<Test>::PostNotFound);
  });
}

#[test]
fn create_comment_should_fail_parent_not_found() {
  new_test_ext().execute_with(|| {
    assert_ok!(_create_default_blog()); // BlogId 1
    assert_ok!(_create_default_post()); // PostId 1

    // Try to catch an error creating a comment with wrong parent
    assert_noop!(_create_comment(None, None, Some(Some(2)), None), Error::<Test>::UnknownParentComment);
  });
}

#[test]
fn create_comment_should_fail_invalid_ipfs_hash() {
  let ipfs_hash : Vec<u8> = b"QmV9tSDx9UiPeWExXEeH6aoDvmihvx6j".to_vec();

  new_test_ext().execute_with(|| {
    assert_ok!(_create_default_blog()); // BlogId 1
    assert_ok!(_create_default_post()); // PostId 1

    // Try to catch an error creating a comment with wrong parent
    assert_noop!(_create_comment(None, None, None, Some(ipfs_hash)), Error::<Test>::IpfsIsIncorrect);
  });
}

#[test]
fn update_comment_should_work() {
  new_test_ext().execute_with(|| {
    assert_ok!(_create_default_blog()); // BlogId 1
    assert_ok!(_create_default_post()); // PostId 1
    assert_ok!(_create_default_comment()); // PostId 2

    // Post update with ID 1 should be fine
    assert_ok!(_update_comment(None, None, None));

    // Check whether post updates correctly
    let comment = Social::post_by_id(2).unwrap();
    assert_eq!(comment.ipfs_hash, self::subcomment_ipfs_hash());

    // Check whether history recorded correctly
    assert_eq!(comment.edit_history[0].old_data.ipfs_hash, Some(self::comment_ipfs_hash()));
  });
}

#[test]
fn update_comment_should_fail_comment_not_found() {
  new_test_ext().execute_with(|| {
    // Try to catch an error updating a comment with wrong PostId
    assert_noop!(_update_comment(None, None, None), Error::<Test>::PostNotFound);
  });
}

#[test]
fn update_comment_should_fail_not_an_owner() {
  new_test_ext().execute_with(|| {
    assert_ok!(_create_default_blog()); // BlogId 1
    assert_ok!(_create_default_post()); // PostId 1
    assert_ok!(_create_default_comment()); // PostId 2

    // Try to catch an error updating a comment with wrong Account
    assert_noop!(_update_comment(
      Some(Origin::signed(2)), None, None
    ), Error::<Test>::NotAnAuthor);
  });
}

#[test]
fn update_comment_should_fail_invalid_ipfs_hash() {
  let ipfs_hash : Vec<u8> = b"QmV9tSDx9UiPeWExXEeH6aoDvmihvx6j".to_vec();

  new_test_ext().execute_with(|| {
    assert_ok!(_create_default_blog()); // BlogId 1
    assert_ok!(_create_default_post()); // PostId 1
    assert_ok!(_create_default_comment()); // PostId 2

    // Try to catch an error updating a comment with invalid ipfs_hash
    assert_noop!(_update_comment(
      None, None, Some(self::post_update(None, Some(ipfs_hash)))
    ), Error::<Test>::IpfsIsIncorrect);
  });
}

#[test]
fn update_comment_should_fail_ipfs_hash_dont_differ() {
  new_test_ext().execute_with(|| {
    assert_ok!(_create_default_blog()); // BlogId 1
    assert_ok!(_create_default_post()); // PostId 1
    assert_ok!(_create_default_comment()); // PostId 2

    // Try to catch an error updating a comment with the same ipfs_hash
    assert_noop!(_update_comment(
      None, None, Some(self::post_update(None, Some(self::comment_ipfs_hash())))
    ), Error::<Test>::CommentIPFSHashNotDiffer);
  });
}

// Reaction tests
#[test]
fn create_post_reaction_should_work_upvote() {
  new_test_ext().execute_with(|| {
    assert_ok!(_create_default_blog()); // BlogId 1
    assert_ok!(_create_default_post()); // PostId 1

    assert_ok!(_create_post_reaction(Some(Origin::signed(ACCOUNT2)), None, None)); // ReactionId 1 by ACCOUNT2

    // Check storages
    assert_eq!(Social::reaction_ids_by_post_id(1), vec![1]);
    assert_eq!(Social::next_reaction_id(), 2);

    // Check post reaction counters
    let post = Social::post_by_id(1).unwrap();
    assert_eq!(post.upvotes_count, 1);
    assert_eq!(post.downvotes_count, 0);

    // Check whether data stored correctly
    let reaction = Social::reaction_by_id(1).unwrap();
    assert_eq!(reaction.created.account, ACCOUNT2);
    assert_eq!(reaction.kind, self::reaction_upvote());
  });
}

#[test]
fn create_post_reaction_should_work_downvote() {
  new_test_ext().execute_with(|| {
    assert_ok!(_create_default_blog()); // BlogId 1
    assert_ok!(_create_default_post()); // PostId 1

    assert_ok!(_create_post_reaction(Some(Origin::signed(ACCOUNT2)), None, Some(self::reaction_downvote()))); // ReactionId 1 by ACCOUNT2

    // Check storages
    assert_eq!(Social::reaction_ids_by_post_id(1), vec![1]);
    assert_eq!(Social::next_reaction_id(), 2);

    // Check post reaction counters
    let post = Social::post_by_id(1).unwrap();
    assert_eq!(post.upvotes_count, 0);
    assert_eq!(post.downvotes_count, 1);

    // Check whether data stored correctly
    let reaction = Social::reaction_by_id(1).unwrap();
    assert_eq!(reaction.created.account, ACCOUNT2);
    assert_eq!(reaction.kind, self::reaction_downvote());
  });
}

#[test]
fn create_post_reaction_should_fail_already_reacted() {
  new_test_ext().execute_with(|| {
    assert_ok!(_create_default_blog()); // BlogId 1
    assert_ok!(_create_default_post()); // PostId 1
    assert_ok!(_create_default_post_reaction()); // ReactionId1

    // Try to catch an error creating reaction by the same account
    assert_noop!(_create_default_post_reaction(), Error::<Test>::AccountAlreadyReactedToPost);
  });
}

#[test]
fn create_post_reaction_should_fail_post_not_found() {
  new_test_ext().execute_with(|| {
    // Try to catch an error creating reaction by the same account
    assert_noop!(_create_default_post_reaction(), Error::<Test>::PostNotFound);
  });
}

#[test]
fn create_comment_reaction_should_work_upvote() {
  new_test_ext().execute_with(|| {
    assert_ok!(_create_default_blog()); // BlogId 1
    assert_ok!(_create_default_post()); // PostId 1
    assert_ok!(_create_default_comment()); // PostId 2
    assert_ok!(_create_comment_reaction(Some(Origin::signed(ACCOUNT2)), None, None)); // ReactionId 1 by ACCOUNT2

    // Check storages
    assert_eq!(Social::reaction_ids_by_comment_id(2), vec![1]);
    assert_eq!(Social::next_reaction_id(), 2);

    // Check comment reaction counters
    let comment = Social::post_by_id(2).unwrap();
    assert_eq!(comment.upvotes_count, 1);
    assert_eq!(comment.downvotes_count, 0);

    // Check whether data stored correctly
    let reaction = Social::reaction_by_id(1).unwrap();
    assert_eq!(reaction.created.account, ACCOUNT2);
    assert_eq!(reaction.kind, self::reaction_upvote());
  });
}

#[test]
fn create_comment_reaction_should_work_downvote() {
  new_test_ext().execute_with(|| {
    assert_ok!(_create_default_blog()); // BlogId 1
    assert_ok!(_create_default_post()); // PostId 1
    assert_ok!(_create_default_comment()); // PostId 2
    assert_ok!(_create_comment_reaction(Some(Origin::signed(ACCOUNT2)), None, Some(self::reaction_downvote()))); // ReactionId 1 by ACCOUNT2

    // Check storages
    assert_eq!(Social::reaction_ids_by_comment_id(2), vec![1]);
    assert_eq!(Social::next_reaction_id(), 2);

    // Check comment reaction counters
    let comment = Social::post_by_id(2).unwrap();
    assert_eq!(comment.upvotes_count, 0);
    assert_eq!(comment.downvotes_count, 1);

    // Check whether data stored correctly
    let reaction = Social::reaction_by_id(1).unwrap();
    assert_eq!(reaction.created.account, ACCOUNT2);
    assert_eq!(reaction.kind, self::reaction_downvote());
  });
}

#[test]
fn create_comment_reaction_should_fail_already_reacted() {
  new_test_ext().execute_with(|| {
    assert_ok!(_create_default_blog()); // BlogId 1
    assert_ok!(_create_default_post()); // PostId 1
    assert_ok!(_create_default_comment()); // PostId 2
    assert_ok!(_create_default_comment_reaction()); // ReactionId 1

    // Try to catch an error creating reaction by the same account
    assert_noop!(_create_default_comment_reaction(), Error::<Test>::AccountAlreadyReactedToComment);
  });
}

#[test]
fn create_comment_reaction_should_fail_comment_not_found() {
  new_test_ext().execute_with(|| {
    // Try to catch an error creating reaction by the same account
    assert_noop!(_create_default_comment_reaction(), Error::<Test>::PostNotFound);
  });
}

// Rating system tests

#[test]
fn score_diff_by_weights_check_result() {
  new_test_ext().execute_with(|| {
    assert_eq!(Social::get_score_diff(1, self::scoring_action_upvote_post()), DEFAULT_UPVOTE_POST_ACTION_WEIGHT as i16);
    assert_eq!(Social::get_score_diff(1, self::scoring_action_downvote_post()), DEFAULT_DOWNVOTE_POST_ACTION_WEIGHT as i16);
    assert_eq!(Social::get_score_diff(1, self::scoring_action_share_post()), DEFAULT_SHARE_POST_ACTION_WEIGHT as i16);
    assert_eq!(Social::get_score_diff(1, self::scoring_action_create_comment()), DEFAULT_CREATE_COMMENT_ACTION_WEIGHT as i16);
    assert_eq!(Social::get_score_diff(1, self::scoring_action_upvote_comment()), DEFAULT_UPVOTE_COMMENT_ACTION_WEIGHT as i16);
    assert_eq!(Social::get_score_diff(1, self::scoring_action_downvote_comment()), DEFAULT_DOWNVOTE_COMMENT_ACTION_WEIGHT as i16);
    assert_eq!(Social::get_score_diff(1, self::scoring_action_share_comment()), DEFAULT_SHARE_COMMENT_ACTION_WEIGHT as i16);
    assert_eq!(Social::get_score_diff(1, self::scoring_action_follow_blog()), DEFAULT_FOLLOW_BLOG_ACTION_WEIGHT as i16);
    assert_eq!(Social::get_score_diff(1, self::scoring_action_follow_account()), DEFAULT_FOLLOW_ACCOUNT_ACTION_WEIGHT as i16);
  });
}

#[test]
fn random_score_diff_check_result() {
  new_test_ext().execute_with(|| {
    assert_eq!(Social::get_score_diff(32768, self::scoring_action_upvote_post()), 80); // 2^15
    assert_eq!(Social::get_score_diff(32769, self::scoring_action_upvote_post()), 80); // 2^15 + 1
    assert_eq!(Social::get_score_diff(65535, self::scoring_action_upvote_post()), 80); // 2^16 - 1
    assert_eq!(Social::get_score_diff(65536, self::scoring_action_upvote_post()), 85); // 2^16
  });
}

//--------------------------------------------------------------------------------------------------

#[test]
fn change_blog_score_should_work_follow_blog() {
  new_test_ext().execute_with(|| {
    assert_ok!(_create_default_blog()); // BlogId 1

    assert_ok!(Social::follow_blog(Origin::signed(ACCOUNT2), 1));

    assert_eq!(Social::blog_by_id(1).unwrap().score, DEFAULT_FOLLOW_BLOG_ACTION_WEIGHT as i32);
    assert_eq!(Social::social_account_by_id(ACCOUNT1).unwrap().reputation, 1 + DEFAULT_FOLLOW_BLOG_ACTION_WEIGHT as u32);
    assert_eq!(Social::social_account_by_id(ACCOUNT2).unwrap().reputation, 1);
  });
}

#[test]
fn change_blog_score_should_work_revert_follow_blog() {
  new_test_ext().execute_with(|| {
    assert_ok!(_create_default_blog()); // BlogId 1

    assert_ok!(Social::follow_blog(Origin::signed(ACCOUNT2), 1));
    assert_ok!(Social::unfollow_blog(Origin::signed(ACCOUNT2), 1));

    assert_eq!(Social::blog_by_id(1).unwrap().score, 0);
    assert_eq!(Social::social_account_by_id(ACCOUNT1).unwrap().reputation, 1);
    assert_eq!(Social::social_account_by_id(ACCOUNT2).unwrap().reputation, 1);
  });
}

#[test]
fn change_blog_score_should_work_upvote_post() {
  new_test_ext().execute_with(|| {
    assert_ok!(_create_default_blog()); // BlogId 1
    assert_ok!(_create_default_post());
    assert_ok!(_create_post_reaction(Some(Origin::signed(ACCOUNT2)), None, None)); // ReactionId 1

    assert_eq!(Social::blog_by_id(1).unwrap().score, DEFAULT_UPVOTE_POST_ACTION_WEIGHT as i32);
    assert_eq!(Social::social_account_by_id(ACCOUNT1).unwrap().reputation, 1 + DEFAULT_UPVOTE_POST_ACTION_WEIGHT as u32);
  });
}

#[test]
fn change_blog_score_should_work_downvote_post() {
  new_test_ext().execute_with(|| {
    assert_ok!(_create_default_blog()); // BlogId 1
    assert_ok!(_create_default_post());
    assert_ok!(_create_post_reaction(Some(Origin::signed(ACCOUNT2)), None, Some(self::reaction_downvote()))); // ReactionId 1

    assert_eq!(Social::blog_by_id(1).unwrap().score, DEFAULT_DOWNVOTE_POST_ACTION_WEIGHT as i32);
    assert_eq!(Social::social_account_by_id(ACCOUNT1).unwrap().reputation, 1);
  });
}

//--------------------------------------------------------------------------------------------------

#[test]
fn change_post_score_should_work_create_comment() {
  new_test_ext().execute_with(|| {
    assert_ok!(_create_default_blog()); // BlogId 1
    assert_ok!(_create_default_post()); // PostId 1
    assert_ok!(_create_comment(Some(Origin::signed(ACCOUNT2)), None, None, None)); // PostId 2

    assert_eq!(Social::post_by_id(1).unwrap().score, DEFAULT_CREATE_COMMENT_ACTION_WEIGHT as i32);
    assert_eq!(Social::blog_by_id(1).unwrap().score, DEFAULT_CREATE_COMMENT_ACTION_WEIGHT as i32);
    assert_eq!(Social::social_account_by_id(ACCOUNT1).unwrap().reputation, 1 + DEFAULT_CREATE_COMMENT_ACTION_WEIGHT as u32);
    assert_eq!(Social::post_score_by_account((ACCOUNT2, 1, self::scoring_action_create_comment())), Some(DEFAULT_CREATE_COMMENT_ACTION_WEIGHT));
  });
}

#[test]
fn change_post_score_should_work_upvote() {
  new_test_ext().execute_with(|| {
    assert_ok!(_create_default_blog());
    assert_ok!(_create_default_post()); // PostId 1

    assert_ok!(_create_post_reaction(Some(Origin::signed(ACCOUNT2)), None, None));

    assert_eq!(Social::post_by_id(1).unwrap().score, DEFAULT_UPVOTE_POST_ACTION_WEIGHT as i32);
    assert_eq!(Social::social_account_by_id(ACCOUNT1).unwrap().reputation, 1 + DEFAULT_UPVOTE_POST_ACTION_WEIGHT as u32);
    assert_eq!(Social::post_score_by_account((ACCOUNT2, 1, self::scoring_action_upvote_post())), Some(DEFAULT_UPVOTE_POST_ACTION_WEIGHT));
  });
}

#[test]
fn change_post_score_should_work_downvote() {
  new_test_ext().execute_with(|| {
    assert_ok!(_create_default_blog());
    assert_ok!(_create_default_post()); // PostId 1

    assert_ok!(_create_post_reaction(Some(Origin::signed(ACCOUNT2)), None, Some(self::reaction_downvote())));

    assert_eq!(Social::post_by_id(1).unwrap().score, DEFAULT_DOWNVOTE_POST_ACTION_WEIGHT as i32);
    assert_eq!(Social::social_account_by_id(ACCOUNT1).unwrap().reputation, 1);
    assert_eq!(Social::post_score_by_account((ACCOUNT2, 1, self::scoring_action_downvote_post())), Some(DEFAULT_DOWNVOTE_POST_ACTION_WEIGHT));
  });
}

#[test]
fn change_post_score_should_revert_upvote() {
  new_test_ext().execute_with(|| {
    assert_ok!(_create_default_blog());
    assert_ok!(_create_default_post()); // PostId 1

    assert_ok!(_create_post_reaction(Some(Origin::signed(ACCOUNT2)), None, None)); // ReactionId 1
    assert_ok!(_delete_post_reaction(Some(Origin::signed(ACCOUNT2)), None, 1));

    assert_eq!(Social::post_by_id(1).unwrap().score, 0);
    assert_eq!(Social::social_account_by_id(ACCOUNT1).unwrap().reputation, 1);
    assert_eq!(Social::post_score_by_account((ACCOUNT2, 1, self::scoring_action_upvote_post())), None);
  });
}

#[test]
fn change_post_score_should_revert_downvote() {
  new_test_ext().execute_with(|| {
    assert_ok!(_create_default_blog());
    assert_ok!(_create_default_post()); // PostId 1

    assert_ok!(_create_post_reaction(Some(Origin::signed(ACCOUNT2)), None, Some(self::reaction_downvote()))); // ReactionId 1
    assert_ok!(_delete_post_reaction(Some(Origin::signed(ACCOUNT2)), None, 1));

    assert_eq!(Social::post_by_id(1).unwrap().score, 0);
    assert_eq!(Social::social_account_by_id(ACCOUNT1).unwrap().reputation, 1);
    assert_eq!(Social::post_score_by_account((ACCOUNT2, 1, self::scoring_action_downvote_post())), None);
  });
}

#[test]
fn change_post_score_cancel_upvote_with_downvote() {
  new_test_ext().execute_with(|| {
    assert_ok!(_create_default_blog());
    assert_ok!(_create_default_post()); // PostId 1

    assert_ok!(_create_post_reaction(Some(Origin::signed(ACCOUNT2)), None, None)); // ReactionId 1
    assert_ok!(_update_post_reaction(Some(Origin::signed(ACCOUNT2)), None, 1, Some(self::reaction_downvote())));

    assert_eq!(Social::post_by_id(1).unwrap().score, DEFAULT_DOWNVOTE_POST_ACTION_WEIGHT as i32);
    assert_eq!(Social::social_account_by_id(ACCOUNT1).unwrap().reputation, 1);
    assert_eq!(Social::post_score_by_account((ACCOUNT2, 1, self::scoring_action_upvote_post())), None);
    assert_eq!(Social::post_score_by_account((ACCOUNT2, 1, self::scoring_action_downvote_post())), Some(DEFAULT_DOWNVOTE_POST_ACTION_WEIGHT));
  });
}

#[test]
fn change_post_score_cancel_downvote_with_upvote() {
  new_test_ext().execute_with(|| {
    assert_ok!(_create_default_blog());
    assert_ok!(_create_default_post()); // PostId 1

    assert_ok!(_create_post_reaction(Some(Origin::signed(ACCOUNT2)), None, Some(self::reaction_downvote()))); // ReactionId 1
    assert_ok!(_update_post_reaction(Some(Origin::signed(ACCOUNT2)), None, 1, None));

    assert_eq!(Social::post_by_id(1).unwrap().score, DEFAULT_UPVOTE_POST_ACTION_WEIGHT as i32);
    assert_eq!(Social::social_account_by_id(ACCOUNT1).unwrap().reputation, 1 + DEFAULT_UPVOTE_POST_ACTION_WEIGHT as u32);
    assert_eq!(Social::post_score_by_account((ACCOUNT2, 1, self::scoring_action_downvote_post())), None);
    assert_eq!(Social::post_score_by_account((ACCOUNT2, 1, self::scoring_action_upvote_post())), Some(DEFAULT_UPVOTE_POST_ACTION_WEIGHT));
  });
}

#[test]
fn change_post_score_should_fail_post_not_found() {
  new_test_ext().execute_with(|| {
    let ref mut fake_post: Post<Test> = self::fake_post(
      1,
      ACCOUNT1,
      None,
      PostExtension::RegularPost
    );

    assert_ok!(_create_default_blog()); // BlogId 1
    assert_noop!(_change_post_score_by_extension(
      ACCOUNT1, fake_post, self::scoring_action_upvote_post()
    ), Error::<Test>::PostNotFound);
  });
}

//--------------------------------------------------------------------------------------------------

#[test]
fn change_social_account_reputation_should_work_max_score_diff() {
  new_test_ext().execute_with(|| {
    assert_ok!(_create_default_blog());
    assert_ok!(_create_post(Some(Origin::signed(ACCOUNT2)), None, None, None));
    assert_ok!(Social::change_social_account_reputation(
      ACCOUNT2,
      ACCOUNT1,
      std::i16::MAX,
      self::scoring_action_follow_account())
    );
  });
}

#[test]
fn change_social_account_reputation_should_work_min_score_diff() {
  new_test_ext().execute_with(|| {
    assert_ok!(_create_default_blog());
    assert_ok!(_create_post(Some(Origin::signed(ACCOUNT2)), None, None, None));
    assert_ok!(Social::change_social_account_reputation(
      ACCOUNT2,
      ACCOUNT1,
      std::i16::MIN,
      self::scoring_action_follow_account())
    );
  });
}

#[test]
fn change_social_account_reputation_should_work() {
  new_test_ext().execute_with(|| {
    assert_ok!(_create_default_blog());
    assert_ok!(_create_post(Some(Origin::signed(ACCOUNT2)), None, None, None));
    assert_ok!(Social::change_social_account_reputation(
      ACCOUNT2,
      ACCOUNT1,
      DEFAULT_DOWNVOTE_POST_ACTION_WEIGHT,
      self::scoring_action_downvote_post())
    );
    assert_eq!(Social::account_reputation_diff_by_account((ACCOUNT1, ACCOUNT2, self::scoring_action_downvote_post())), Some(0));
    assert_eq!(Social::social_account_by_id(ACCOUNT2).unwrap().reputation, 1);

    assert_ok!(Social::change_social_account_reputation(
      ACCOUNT2,
      ACCOUNT1,
      DEFAULT_UPVOTE_POST_ACTION_WEIGHT * 2,
      self::scoring_action_upvote_post())
    );
    assert_eq!(Social::account_reputation_diff_by_account(
      (ACCOUNT1, ACCOUNT2, self::scoring_action_upvote_post())),
               Some(DEFAULT_UPVOTE_POST_ACTION_WEIGHT * 2)
    );
    assert_eq!(Social::social_account_by_id(ACCOUNT2).unwrap().reputation, 1 + (DEFAULT_UPVOTE_POST_ACTION_WEIGHT * 2) as u32);
  });
}

//--------------------------------------------------------------------------------------------------

#[test]
fn change_comment_score_should_work_upvote() {
  new_test_ext().execute_with(|| {
    assert_ok!(_create_default_blog()); // BlogId 1
    assert_ok!(_create_post(Some(Origin::signed(ACCOUNT2)), None, None, None)); // PostId 1
    assert_ok!(_create_comment(Some(Origin::signed(ACCOUNT2)), None, None, None)); // PostId 2

    assert_ok!(_change_post_score_by_extension_with_id(ACCOUNT1, 2, self::scoring_action_upvote_comment()));

    assert_eq!(Social::post_by_id(2).unwrap().score, DEFAULT_UPVOTE_COMMENT_ACTION_WEIGHT as i32);
    assert_eq!(Social::social_account_by_id(ACCOUNT1).unwrap().reputation, 1);
    assert_eq!(Social::social_account_by_id(ACCOUNT2).unwrap().reputation, 1 + DEFAULT_UPVOTE_COMMENT_ACTION_WEIGHT as u32);
    assert_eq!(Social::comment_score_by_account((ACCOUNT1, 2, self::scoring_action_upvote_comment())), Some(DEFAULT_UPVOTE_COMMENT_ACTION_WEIGHT));
  });
}

#[test]
fn change_comment_score_should_work_downvote() {
  new_test_ext().execute_with(|| {
    assert_ok!(_create_default_blog()); // BlogId 1
    assert_ok!(_create_post(Some(Origin::signed(ACCOUNT2)), None, None, None)); // PostId 1
    assert_ok!(_create_comment(Some(Origin::signed(ACCOUNT2)), None, None, None)); // PostId 2

    assert_ok!(_change_post_score_by_extension_with_id(ACCOUNT1, 2, self::scoring_action_downvote_comment()));

    assert_eq!(Social::post_by_id(2).unwrap().score, DEFAULT_DOWNVOTE_COMMENT_ACTION_WEIGHT as i32);
    assert_eq!(Social::social_account_by_id(ACCOUNT1).unwrap().reputation, 1);
    assert_eq!(Social::social_account_by_id(ACCOUNT2).unwrap().reputation, 1);
    assert_eq!(Social::comment_score_by_account((ACCOUNT1, 2, self::scoring_action_downvote_comment())), Some(DEFAULT_DOWNVOTE_COMMENT_ACTION_WEIGHT));
  });
}

#[test]
fn change_comment_score_should_revert_upvote() {
  new_test_ext().execute_with(|| {
    assert_ok!(_create_default_blog()); // BlogId 1
    assert_ok!(_create_post(Some(Origin::signed(ACCOUNT2)), None, None, None)); // PostId 1
    assert_ok!(_create_comment(Some(Origin::signed(ACCOUNT2)), None, None, None)); // PostId 2

    assert_ok!(_change_post_score_by_extension_with_id(ACCOUNT1, 2, self::scoring_action_upvote_comment()));
    assert_ok!(_change_post_score_by_extension_with_id(ACCOUNT1, 2, self::scoring_action_upvote_comment()));

    assert_eq!(Social::post_by_id(2).unwrap().score, 0);
    assert_eq!(Social::social_account_by_id(ACCOUNT1).unwrap().reputation, 1);
    assert_eq!(Social::social_account_by_id(ACCOUNT2).unwrap().reputation, 1);
    assert_eq!(Social::comment_score_by_account((ACCOUNT1, 2, self::scoring_action_upvote_comment())), None);
  });
}

#[test]
fn change_comment_score_should_revert_downvote() {
  new_test_ext().execute_with(|| {
    assert_ok!(_create_default_blog()); // BlogId 1
    assert_ok!(_create_post(Some(Origin::signed(ACCOUNT2)), None, None, None)); // PostId 1
    assert_ok!(_create_comment(Some(Origin::signed(ACCOUNT2)), None, None, None)); // PostId 2

    assert_ok!(_change_post_score_by_extension_with_id(ACCOUNT1, 2, self::scoring_action_downvote_comment()));
    assert_ok!(_change_post_score_by_extension_with_id(ACCOUNT1, 2, self::scoring_action_downvote_comment()));

    assert_eq!(Social::post_by_id(2).unwrap().score, 0);
    assert_eq!(Social::social_account_by_id(ACCOUNT1).unwrap().reputation, 1);
    assert_eq!(Social::social_account_by_id(ACCOUNT2).unwrap().reputation, 1);
    assert_eq!(Social::comment_score_by_account((ACCOUNT1, 2, self::scoring_action_downvote_comment())), None);
  });
}

#[test]
fn change_comment_score_check_cancel_upvote() {
  new_test_ext().execute_with(|| {
    assert_ok!(_create_default_blog()); // BlogId 1
    assert_ok!(_create_post(Some(Origin::signed(ACCOUNT2)), None, None, None)); // PostId 1
    assert_ok!(_create_comment(Some(Origin::signed(ACCOUNT2)), None, None, None)); // PostId 2

    assert_ok!(_change_post_score_by_extension_with_id(ACCOUNT1, 2, self::scoring_action_upvote_comment()));
    assert_ok!(_change_post_score_by_extension_with_id(ACCOUNT1, 2, self::scoring_action_downvote_comment()));

    assert_eq!(Social::post_by_id(2).unwrap().score, DEFAULT_DOWNVOTE_COMMENT_ACTION_WEIGHT as i32);
    assert_eq!(Social::social_account_by_id(ACCOUNT1).unwrap().reputation, 1);
    assert_eq!(Social::social_account_by_id(ACCOUNT2).unwrap().reputation, 1);
    assert_eq!(Social::comment_score_by_account((ACCOUNT1, 2, self::scoring_action_upvote_comment())), None);
    assert_eq!(Social::comment_score_by_account((ACCOUNT1, 2, self::scoring_action_downvote_comment())), Some(DEFAULT_DOWNVOTE_COMMENT_ACTION_WEIGHT));
  });
}

#[test]
fn change_comment_score_check_cancel_downvote() {
  new_test_ext().execute_with(|| {
    assert_ok!(_create_default_blog()); // BlogId 1
    assert_ok!(_create_post(Some(Origin::signed(ACCOUNT2)), None, None, None)); // PostId 1
    assert_ok!(_create_comment(Some(Origin::signed(ACCOUNT2)), None, None, None)); // PostId 2

    assert_ok!(_change_post_score_by_extension_with_id(ACCOUNT1, 2, self::scoring_action_downvote_comment()));
    assert_ok!(_change_post_score_by_extension_with_id(ACCOUNT1, 2, self::scoring_action_upvote_comment()));

    assert_eq!(Social::post_by_id(2).unwrap().score, DEFAULT_UPVOTE_COMMENT_ACTION_WEIGHT as i32);
    assert_eq!(Social::social_account_by_id(ACCOUNT1).unwrap().reputation, 1);
    assert_eq!(Social::social_account_by_id(ACCOUNT2).unwrap().reputation, 1 + DEFAULT_UPVOTE_COMMENT_ACTION_WEIGHT as u32);
    assert_eq!(Social::comment_score_by_account((ACCOUNT1, 2, self::scoring_action_downvote_comment())), None);
    assert_eq!(Social::comment_score_by_account((ACCOUNT1, 2, self::scoring_action_upvote_comment())), Some(DEFAULT_UPVOTE_COMMENT_ACTION_WEIGHT));
  });
}

#[test]
fn change_comment_score_should_fail_comment_not_found() {
  new_test_ext().execute_with(|| {
    let ref mut fake_post: Post<Test> = self::fake_post(
      3,
      ACCOUNT1,
      None,
      PostExtension::Comment(CommentExt{
        parent_id: None,
        root_post_id: 1
      })
    );

    assert_ok!(_create_default_blog()); // BlogId 1
    assert_ok!(_create_default_post()); // PostId 1
    assert_ok!(_create_comment(Some(Origin::signed(ACCOUNT2)), None, None, None)); // PostId 2

    assert_noop!(_change_post_score_by_extension(
      ACCOUNT1, fake_post, self::scoring_action_upvote_comment()
    ), Error::<Test>::PostNotFound);
  });
}

// Shares tests

#[test]
fn share_post_should_work() {
  new_test_ext().execute_with(|| {
    assert_ok!(_create_default_blog()); // BlogId 1
    assert_ok!(_create_blog(Some(Origin::signed(ACCOUNT2)), Some(Some(b"blog2_handle".to_vec())), None)); // BlogId 2 by ACCOUNT2
    assert_ok!(_create_default_post()); // PostId 1
    assert_ok!(_create_post(
      Some(Origin::signed(ACCOUNT2)),
      Some(Some(2)),
      Some(self::extension_shared_post(1)),
      Some(vec![])
    )); // Share PostId 1 on BlogId 2 by ACCOUNT2

    // Check storages
    assert_eq!(Social::post_ids_by_blog_id(1), vec![1]);
    assert_eq!(Social::post_ids_by_blog_id(2), vec![2]);
    assert_eq!(Social::next_post_id(), 3);

    assert_eq!(Social::post_shares_by_account((ACCOUNT2, 1)), 1);
    assert_eq!(Social::shared_post_ids_by_original_post_id(1), vec![2]);

    // Check whether data stored correctly
    assert_eq!(Social::post_by_id(1).unwrap().shares_count, 1);

    let shared_post = Social::post_by_id(2).unwrap();

    assert_eq!(shared_post.blog_id, Some(2));
    assert_eq!(shared_post.created.account, ACCOUNT2);
    assert!(shared_post.ipfs_hash.is_empty());
    assert_eq!(shared_post.extension, self::extension_shared_post(1));
  });
}

#[test]
fn share_post_should_work_share_own_post() {
  new_test_ext().execute_with(|| {
    assert_ok!(_create_default_blog()); // BlogId 1
    assert_ok!(_create_default_post()); // PostId 1
    assert_ok!(_create_post(
      Some(Origin::signed(ACCOUNT1)),
      Some(Some(1)),
      Some(self::extension_shared_post(1)),
      Some(vec![])
    )); // Share PostId 1

    // Check storages
    assert_eq!(Social::post_ids_by_blog_id(1), vec![1, 2]);
    assert_eq!(Social::next_post_id(), 3);

    assert_eq!(Social::post_shares_by_account((ACCOUNT1, 1)), 1);
    assert_eq!(Social::shared_post_ids_by_original_post_id(1), vec![2]);

    // Check whether data stored correctly
    assert_eq!(Social::post_by_id(1).unwrap().shares_count, 1);

    let shared_post = Social::post_by_id(2).unwrap();
    assert_eq!(shared_post.blog_id, Some(1));
    assert_eq!(shared_post.created.account, ACCOUNT1);
    assert!(shared_post.ipfs_hash.is_empty());
    assert_eq!(shared_post.extension, self::extension_shared_post(1));
  });
}

#[test]
fn share_post_should_change_score() {
  new_test_ext().execute_with(|| {
    assert_ok!(_create_default_blog()); // BlogId 1
    assert_ok!(_create_blog(Some(Origin::signed(ACCOUNT2)), Some(Some(b"blog2_handle".to_vec())), None)); // BlogId 2 by ACCOUNT2
    assert_ok!(_create_default_post()); // PostId 1
    assert_ok!(_create_post(
      Some(Origin::signed(ACCOUNT2)),
      Some(Some(2)),
      Some(self::extension_shared_post(1)),
      Some(vec![])
    )); // Share PostId 1 on BlogId 2 by ACCOUNT2

    assert_eq!(Social::post_by_id(1).unwrap().score, DEFAULT_SHARE_POST_ACTION_WEIGHT as i32);
    assert_eq!(Social::social_account_by_id(ACCOUNT1).unwrap().reputation, 1 + DEFAULT_SHARE_POST_ACTION_WEIGHT as u32);
    assert_eq!(Social::post_score_by_account((ACCOUNT2, 1, self::scoring_action_share_post())), Some(DEFAULT_SHARE_POST_ACTION_WEIGHT));
  });
}

#[test]
fn share_post_should_not_change_score() {
  new_test_ext().execute_with(|| {
    assert_ok!(_create_default_blog()); // BlogId 1
    assert_ok!(_create_default_post()); // PostId 1
    assert_ok!(_create_post(
      Some(Origin::signed(ACCOUNT1)),
      Some(Some(1)),
      Some(self::extension_shared_post(1)),
      Some(vec![])
    )); // Share PostId

    assert_eq!(Social::post_by_id(1).unwrap().score, 0);
    assert_eq!(Social::social_account_by_id(ACCOUNT1).unwrap().reputation, 1);
    assert_eq!(Social::post_score_by_account((ACCOUNT1, 1, self::scoring_action_share_post())), None);
  });
}

#[test]
fn share_post_should_fail_original_post_not_found() {
  new_test_ext().execute_with(|| {
    assert_ok!(_create_default_blog()); // BlogId 1
    assert_ok!(_create_blog(Some(Origin::signed(ACCOUNT2)), Some(Some(b"blog2_handle".to_vec())), None)); // BlogId 2 by ACCOUNT2
    // Skipped creating PostId 1
    assert_noop!(_create_post(
      Some(Origin::signed(ACCOUNT2)),
      Some(Some(2)),
      Some(self::extension_shared_post(1)),
      Some(vec![])
    ), Error::<Test>::OriginalPostNotFound);
  });
}

#[test]
fn share_post_should_fail_share_shared_post() {
  new_test_ext().execute_with(|| {
    assert_ok!(_create_default_blog()); // BlogId 1
    assert_ok!(_create_blog(Some(Origin::signed(ACCOUNT2)), Some(Some(b"blog2_handle".to_vec())), None)); // BlogId 2 by ACCOUNT2
    assert_ok!(_create_default_post());
    assert_ok!(_create_post(
      Some(Origin::signed(ACCOUNT2)),
      Some(Some(2)),
      Some(self::extension_shared_post(1)),
      Some(vec![]))
    );

    // Try to share post with extension SharedPost
    assert_noop!(_create_post(
      Some(Origin::signed(ACCOUNT1)),
      Some(Some(1)),
      Some(self::extension_shared_post(2)),
      Some(vec![])
    ), Error::<Test>::CannotShareSharedPost);
  });
}

#[test]
fn share_comment_should_work() {
  new_test_ext().execute_with(|| {
    assert_ok!(_create_default_blog()); // BlogId 1
    assert_ok!(_create_blog(Some(Origin::signed(ACCOUNT2)), Some(Some(b"blog2_handle".to_vec())), None)); // BlogId 2 by ACCOUNT2
    assert_ok!(_create_default_post()); // PostId 1
    assert_ok!(_create_default_comment()); // PostId 2
    assert_ok!(_create_post(
      Some(Origin::signed(ACCOUNT2)),
      Some(Some(2)),
      Some(self::extension_shared_post(2)),
      Some(vec![])
    )); // Share PostId 2 comment on BlogId 2 by ACCOUNT2

    // Check storages
    assert_eq!(Social::post_ids_by_blog_id(1), vec![1]);
    assert_eq!(Social::post_ids_by_blog_id(2), vec![3]);
    assert_eq!(Social::next_post_id(), 4);

    assert_eq!(Social::comment_shares_by_account((ACCOUNT2, 2)), 1);
    assert_eq!(Social::shared_post_ids_by_original_comment_id(2), vec![3]);

    // Check whether data stored correctly
    assert_eq!(Social::post_by_id(2).unwrap().shares_count, 1);

    let shared_post = Social::post_by_id(3).unwrap();

    assert_eq!(shared_post.blog_id, Some(2));
    assert_eq!(shared_post.created.account, ACCOUNT2);
    assert!(shared_post.ipfs_hash.is_empty());
    assert_eq!(shared_post.extension, self::extension_shared_post(2));
  });
}

#[test]
fn share_comment_should_change_score() {
  new_test_ext().execute_with(|| {
    assert_ok!(_create_default_blog()); // BlogId 1
    assert_ok!(_create_blog(Some(Origin::signed(ACCOUNT2)), Some(Some(b"blog2_handle".to_vec())), None)); // BlogId 2 by ACCOUNT2
    assert_ok!(_create_default_post()); // PostId 1
    assert_ok!(_create_default_comment()); // PostId 2
    assert_ok!(_create_post(
      Some(Origin::signed(ACCOUNT2)),
      Some(Some(2)),
      Some(self::extension_shared_post(2)),
      Some(vec![])
    )); // Share PostId 2 comment on BlogId 2 by ACCOUNT2

    assert_eq!(Social::post_by_id(2).unwrap().score, DEFAULT_SHARE_COMMENT_ACTION_WEIGHT as i32);
    assert_eq!(Social::social_account_by_id(ACCOUNT1).unwrap().reputation, 1 + DEFAULT_SHARE_COMMENT_ACTION_WEIGHT as u32);
    assert_eq!(Social::comment_score_by_account((ACCOUNT2, 2, self::scoring_action_share_comment())), Some(DEFAULT_SHARE_COMMENT_ACTION_WEIGHT));
  });
}

#[test]
fn share_comment_should_fail_original_comment_not_found() {
  new_test_ext().execute_with(|| {
    assert_ok!(_create_default_blog()); // BlogId 1
    assert_ok!(_create_blog(Some(Origin::signed(ACCOUNT2)), Some(Some(b"blog2_handle".to_vec())), None)); // BlogId 2 by ACCOUNT2
    assert_ok!(_create_default_post()); // PostId 1
    // Skipped creating comment with PostId 2
    assert_noop!(_create_post(
      Some(Origin::signed(ACCOUNT2)),
      Some(Some(2)),
      Some(self::extension_shared_post(2)),
      None
     ), Error::<Test>::OriginalPostNotFound);
  });
}

// Profiles tests

#[test]
fn create_profile_should_work() {
  new_test_ext().execute_with(|| {
    assert_ok!(_create_default_profile()); // AccountId 1

    let profile = Social::social_account_by_id(ACCOUNT1).unwrap().profile.unwrap();
    assert_eq!(profile.created.account, ACCOUNT1);
    // TODO: Fix unresolved error
    // assert_eq!(profile.updated, None);
    assert_eq!(profile.username, self::alice_username());
    assert_eq!(profile.ipfs_hash, self::profile_ipfs_hash());
    assert!(profile.edit_history.is_empty());
    assert_eq!(Social::account_by_profile_username(self::alice_username()), Some(ACCOUNT1));
  });
}

#[test]
fn create_profile_should_fail_profile_exists() {
  new_test_ext().execute_with(|| {
    assert_ok!(_create_default_profile()); // AccountId 1
    assert_noop!(_create_default_profile(), Error::<Test>::ProfileAlreadyExists);
  });
}

#[test]
fn create_profile_should_fail_invalid_ipfs_hash() {
  let ipfs_hash : Vec<u8> = b"QmV9tSDx9UiPeWExXEeH6aoDvmihvx6j".to_vec();

  new_test_ext().execute_with(|| {
    assert_noop!(_create_profile(None, None, Some(ipfs_hash)), Error::<Test>::IpfsIsIncorrect);
  });
}

#[test]
fn create_profile_should_fail_username_is_busy() {
  new_test_ext().execute_with(|| {
    assert_ok!(_create_default_profile()); // AccountId 1
    assert_noop!(_create_profile(Some(Origin::signed(ACCOUNT2)), None, None), Error::<Test>::UsernameIsBusy);
  });
}

#[test]
fn create_profile_should_fail_too_short_username() {
  let username : Vec<u8> = vec![97; (DEFAULT_USERNAME_MIN_LEN - 1) as usize];

  new_test_ext().execute_with(|| {
    assert_ok!(_create_default_profile()); // AccountId 1
    assert_noop!(_create_profile(Some(Origin::signed(ACCOUNT2)), Some(username), None), Error::<Test>::UsernameIsTooShort);
  });
}

#[test]
fn create_profile_should_fail_too_long_username() {
  let username : Vec<u8> = vec![97; (DEFAULT_USERNAME_MAX_LEN + 1) as usize];

  new_test_ext().execute_with(|| {
    assert_ok!(_create_default_profile()); // AccountId 1
    assert_noop!(_create_profile(Some(Origin::signed(ACCOUNT2)), Some(username), None), Error::<Test>::UsernameIsTooLong);
  });
}

#[test]
fn create_profile_should_fail_invalid_username() {
  let username : Vec<u8> = b"{}sername".to_vec();

  new_test_ext().execute_with(|| {
    assert_ok!(_create_default_profile()); // AccountId 1
    assert_noop!(_create_profile(Some(Origin::signed(ACCOUNT2)), Some(username), None), Error::<Test>::UsernameIsNotAlphanumeric);
  });
}

#[test]
fn update_profile_should_work() {
  new_test_ext().execute_with(|| {
    assert_ok!(_create_default_profile()); // AccountId 1
    assert_ok!(_update_profile(None, Some(self::bob_username()), Some(self::blog_ipfs_hash())));

    // Check whether profile updated correctly
    let profile = Social::social_account_by_id(ACCOUNT1).unwrap().profile.unwrap();
    assert!(profile.updated.is_some());
    assert_eq!(profile.username, self::bob_username());
    assert_eq!(profile.ipfs_hash, self::blog_ipfs_hash());

    // Check storages
    assert_eq!(Social::account_by_profile_username(self::alice_username()), None);
    assert_eq!(Social::account_by_profile_username(self::bob_username()), Some(ACCOUNT1));

    // Check whether profile history is written correctly
    assert_eq!(profile.edit_history[0].old_data.username, Some(self::alice_username()));
    assert_eq!(profile.edit_history[0].old_data.ipfs_hash, Some(self::profile_ipfs_hash()));
  });
}

#[test]
fn update_profile_should_fail_no_social_account() {
  new_test_ext().execute_with(|| {
    assert_noop!(_update_profile(None, Some(self::bob_username()), None), Error::<Test>::SocialAccountNotFound);
  });
}

#[test]
fn update_profile_should_fail_no_profile() {
  new_test_ext().execute_with(|| {
    assert_ok!(Social::follow_account(Origin::signed(ACCOUNT1), ACCOUNT2));
    assert_noop!(_update_profile(None, Some(self::bob_username()), None), Error::<Test>::ProfileDoesNotExist);
  });
}

#[test]
fn update_profile_should_fail_nothing_to_update() {
  new_test_ext().execute_with(|| {
    assert_ok!(_create_default_profile()); // AccountId 1
    assert_noop!(_update_profile(None, None, None), Error::<Test>::NoUpdatesInProfile);
  });
}

#[test]
fn update_profile_should_fail_username_is_busy() {
  new_test_ext().execute_with(|| {
    assert_ok!(_create_default_profile()); // AccountId 1
    assert_ok!(_create_profile(Some(Origin::signed(ACCOUNT2)), Some(self::bob_username()), None));
    assert_noop!(_update_profile(None, Some(self::bob_username()), None), Error::<Test>::UsernameIsBusy);
  });
}

#[test]
fn update_profile_should_fail_too_short_username() {
  let username : Vec<u8> = vec![97; (DEFAULT_USERNAME_MIN_LEN - 1) as usize];

  new_test_ext().execute_with(|| {
    assert_ok!(_create_default_profile()); // AccountId 1
    assert_noop!(_update_profile(None, Some(username), None), Error::<Test>::UsernameIsTooShort);
  });
}

#[test]
fn update_profile_should_fail_too_long_username() {
  let username : Vec<u8> = vec![97; (DEFAULT_USERNAME_MAX_LEN + 1) as usize];

  new_test_ext().execute_with(|| {
    assert_ok!(_create_default_profile()); // AccountId 1
    assert_noop!(_update_profile(None, Some(username), None), Error::<Test>::UsernameIsTooLong);
  });
}

#[test]
fn update_profile_should_fail_invalid_username() {
  let username : Vec<u8> = b"{}sername".to_vec();

  new_test_ext().execute_with(|| {
    assert_ok!(_create_default_profile()); // AccountId 1
    assert_noop!(_update_profile(None, Some(username), None), Error::<Test>::UsernameIsNotAlphanumeric);
  });
}

#[test]
fn update_profile_should_fail_invalid_ipfs_hash() {
  let ipfs_hash : Vec<u8> = b"QmV9tSDx9UiPeWExXEeH6aoDvmihvx6j".to_vec();

  new_test_ext().execute_with(|| {
    assert_ok!(_create_default_profile());
    assert_noop!(_update_profile(None, None, Some(ipfs_hash)), Error::<Test>::IpfsIsIncorrect);
  });
}

// Blog following tests

#[test]
fn follow_blog_should_work() {
  new_test_ext().execute_with(|| {
    assert_ok!(_create_default_blog()); // BlogId 1

    assert_ok!(_default_follow_blog()); // Follow BlogId 1 by ACCOUNT2

    assert_eq!(Social::blog_by_id(1).unwrap().followers_count, 2);
    assert_eq!(Social::blogs_followed_by_account(ACCOUNT2), vec![1]);
    assert_eq!(Social::blog_followers(1), vec![ACCOUNT1, ACCOUNT2]);
    assert_eq!(Social::blog_followed_by_account((ACCOUNT2, 1)), true);
  });
}

#[test]
fn follow_blog_should_fail_blog_not_found() {
  new_test_ext().execute_with(|| {
    assert_noop!(_default_follow_blog(), Error::<Test>::BlogNotFound);
  });
}

#[test]
fn follow_blog_should_fail_already_following() {
  new_test_ext().execute_with(|| {
    assert_ok!(_create_default_blog()); // BlogId 1
    assert_ok!(_default_follow_blog()); // Follow BlogId 1 by ACCOUNT2

    assert_noop!(_default_follow_blog(), Error::<Test>::AccountIsFollowingBlog);
  });
}

#[test]
fn unfollow_blog_should_work() {
  new_test_ext().execute_with(|| {
    assert_ok!(_create_default_blog()); // BlogId 1

    assert_ok!(_default_follow_blog()); // Follow BlogId 1 by ACCOUNT2
    assert_ok!(_default_unfollow_blog());

    assert_eq!(Social::blog_by_id(1).unwrap().followers_count, 1);
    assert!(Social::blogs_followed_by_account(ACCOUNT2).is_empty());
    assert_eq!(Social::blog_followers(1), vec![ACCOUNT1]);
  });
}

#[test]
fn unfollow_blog_should_fail_blog_not_found() {
  new_test_ext().execute_with(|| {
    assert_noop!(_default_unfollow_blog(), Error::<Test>::BlogNotFound);
  });
}

#[test]
fn unfollow_blog_should_fail_already_following() {
  new_test_ext().execute_with(|| {
    assert_ok!(_create_default_blog()); // BlogId 1

    assert_noop!(_default_unfollow_blog(), Error::<Test>::AccountIsNotFollowingBlog);
  });
}

// Account following tests

#[test]
fn follow_account_should_work() {
  new_test_ext().execute_with(|| {
    assert_ok!(_default_follow_account()); // Follow ACCOUNT1 by ACCOUNT2

    assert_eq!(Social::accounts_followed_by_account(ACCOUNT2), vec![ACCOUNT1]);
    assert_eq!(Social::account_followers(ACCOUNT1), vec![ACCOUNT2]);
    assert_eq!(Social::account_followed_by_account((ACCOUNT2, ACCOUNT1)), true);
  });
}

#[test]
fn follow_account_should_fail_follow_itself() {
  new_test_ext().execute_with(|| {
    assert_noop!(_follow_account(None, Some(ACCOUNT2)), Error::<Test>::AccountCannotFollowItself);
  });
}

#[test]
fn follow_account_should_fail_already_followed() {
  new_test_ext().execute_with(|| {
    assert_ok!(_default_follow_account());

    assert_noop!(_default_follow_account(), Error::<Test>::AccountIsAlreadyFollowed);
  });
}



#[test]
fn unfollow_account_should_work() {
  new_test_ext().execute_with(|| {
    assert_ok!(_default_follow_account()); // Follow ACCOUNT1 by ACCOUNT2

    assert_eq!(Social::accounts_followed_by_account(ACCOUNT2), vec![ACCOUNT1]);
    assert_eq!(Social::account_followers(ACCOUNT1), vec![ACCOUNT2]);
    assert_eq!(Social::account_followed_by_account((ACCOUNT2, ACCOUNT1)), true);
  });
}

#[test]
fn unfollow_account_should_fail_unfollow_itself() {
  new_test_ext().execute_with(|| {
    assert_noop!(_unfollow_account(None, Some(ACCOUNT2)), Error::<Test>::AccountCannotUnfollowItself);
  });
}

#[test]
fn unfollow_account_should_fail_is_not_followed() {
  new_test_ext().execute_with(|| {
    assert_ok!(_default_follow_account());
    assert_ok!(_default_unfollow_account());

    assert_noop!(_default_unfollow_account(), Error::<Test>::AccountIsNotFollowed);
  });
}
