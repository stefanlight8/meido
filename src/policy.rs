use crate::category::Category;

#[derive(Debug, Clone, Copy)]
pub enum Policy {
    Scan,
    Collect(Category),
}
