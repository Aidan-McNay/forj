// =======================================================================
// mod.rs
// =======================================================================
// Parsing for 1800-2023 A.2

pub mod assertion_declarations;
pub mod block_item_declarations;
pub mod covergroup_declarations;
pub mod declaration_assignments;
pub mod declaration_data_types;
pub mod declaration_lists;
pub mod declaration_ranges;
pub mod declaration_types;
pub mod function_declarations;
pub mod interface_declarations;
pub mod let_declarations;
pub mod task_declarations;
pub use assertion_declarations::*;
pub use block_item_declarations::*;
pub use covergroup_declarations::*;
pub use declaration_assignments::*;
pub use declaration_data_types::*;
pub use declaration_lists::*;
pub use declaration_ranges::*;
pub use declaration_types::*;
pub use function_declarations::*;
pub use interface_declarations::*;
pub use let_declarations::*;
pub use task_declarations::*;
