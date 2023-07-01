use std::collections::HashMap;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ErrorResponse {
    pub error: ErrorDetail
}

#[derive(Debug, Deserialize)]
pub struct ErrorDetail {
    pub id:     String,
    pub name:   String,
    pub detail: String
}

#[derive(Debug, Deserialize)]
pub struct UserData {
    pub user: User
}

#[derive(Debug, Deserialize)]
pub struct UserResponse {
    pub data: UserData
}

#[derive(Debug, Deserialize)]
pub struct User {
    pub id: String
}

#[derive(Debug, Deserialize)]
pub struct DateFormat {
    pub format: String
}

#[derive(Debug, Deserialize)]
pub struct CurrencyFormat {
    pub iso_code:           String,
    pub example_format:	    String,
    pub decimal_digits:     i32,
    pub decimal_separator:	String,
    pub symbol_first:	    bool,
    pub group_separator:    String,
    pub currency_symbol:    String,
    pub display_symbol:     bool
}

#[derive(Debug, Deserialize)]
pub struct BudgetSummaryResponseData {
    pub budgets:        Vec<BudgetSummary>,
    pub default_budget: Option<BudgetSummary>
}

#[derive(Debug, Deserialize)]
pub struct BudgetSummaryResponse {
    pub data: BudgetSummaryResponseData
}

#[derive(Debug, Deserialize)]
pub struct BudgetSummary {
    pub id:                 String,
    pub name:               String,
    pub last_modified_on:   Option<String>,
    pub first_month:        Option<String>,
    pub last_month:         Option<String>,
    pub date_format:        Option<DateFormat>,
    pub currency_format:    Option<CurrencyFormat>,
    pub accounts:           Option<Vec<Account>>
}

#[derive(Debug, Deserialize)]
pub struct BudgetDetailResponseData {
    pub budget:             BudgetDetail,
    pub server_knowledge:   i64
}

#[derive(Debug, Deserialize)]
pub struct BudgetDetailResponse {
    pub data: BudgetDetailResponseData
}

#[derive(Debug, Deserialize)]
pub struct BudgetDetail {
    pub id:                         String,
    pub name:                       String,
    pub last_modified_on:           String,
    pub first_month:                String,
    pub last_month:                 String,
    pub date_format:                DateFormat,
    pub currency_format:            CurrencyFormat,
    pub accounts:                   Vec<Account>,
    pub payees:                     Vec<Payee>,
    pub payee_locations:            Vec<PayeeLocation>,
    pub category_groups:            Vec<CategoryGroup>,
    pub categories:                 Vec<Category>,
    pub months:                     Vec<MonthDetail>,
    pub transactions:               Vec<TransactionSummary>,
    pub subtransactions:            Vec<SubTransaction>,
    pub scheduled_transactions:     Vec<ScheduledTransactionSummary>,
    pub scheduled_subtransactions:  Vec<ScheduledSubTransaction>
}

#[derive(Debug, Deserialize)]
pub struct BudgetSettingsResponseData {
    pub settings: BudgetSettings
}

#[derive(Debug, Deserialize)]
pub struct BudgetSettingsResponse {
    pub data: BudgetSettingsResponseData
}

#[derive(Debug, Deserialize)]
pub struct BudgetSettings {
    pub date_format: DateFormat,
    pub currency_format: CurrencyFormat
}

#[derive(Debug, Deserialize)]
pub struct AccountsResponseData {
    pub accounts: Vec<Account>,
    pub server_knowledge: i64
}

#[derive(Debug, Deserialize)]
pub struct AccountsResponse {
    pub data: AccountsResponseData
}

#[derive(Debug, Deserialize)]
pub struct AccountResponseData {
    pub account: Account
}

#[derive(Debug, Deserialize)]
pub struct AccountResponse {
    pub data: AccountResponseData
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AccountType {
    Checking,
    Savings,
    Cash,
    CreditCard,
    LineOfCredit,
    OtherAsset,
    OtherLiability,
    Mortgage,
    AutoLoan,
    StudentLoan,
    PersonalLoan,
    MedicalDebt,
    OtherDebt
}

#[derive(Debug, Deserialize)]
pub struct Account {
    pub id:                     String,
    pub name:                   String,
    pub r#type:                 AccountType,
    pub on_budget:              bool,
    pub closed:                 bool,
    pub note:                   Option<String>,
    pub balance:                i64,
    pub cleared_balance:        i64,
    pub uncleared_balance:      i64,
    pub transfer_payee_id:      String,
    pub direct_import_linked:   Option<bool>,
    pub direct_import_in_error: Option<bool>,
    pub last_reconciled_at:     Option<String>,
    pub debt_original_balance:  Option<i64>,
    pub debt_interest_rates:    Option<LoanAccountPeriodicValue>,
    pub debt_minimum_payments:  Option<LoanAccountPeriodicValue>,
    pub debt_escrow_amounts:	Option<LoanAccountPeriodicValue>,
    pub deleted:                bool
}

#[derive(Debug, Deserialize)]
pub struct PostAccountWrapper {
    pub account: SaveAccount
}

#[derive(Debug, Deserialize)]
pub struct SaveAccount {
    pub name: String,
    pub account_type: AccountType,
    pub balance: i64
}

#[derive(Debug, Deserialize)]
pub struct LoanAccountPeriodicValue {
    #[serde(flatten)]
    pub map: HashMap<String, i64>
}

#[derive(Debug, Deserialize)]
pub struct CategoriesResponseData {
    pub category_groups: Vec<CategoryGroupWithCategories>,
    pub server_knowledge: i64,
}

#[derive(Debug, Deserialize)]
pub struct CategoriesResponse {
    pub data: CategoriesResponseData
}

#[derive(Debug, Deserialize)]
pub struct CategoryResponseData {
    pub category: Category
}

#[derive(Debug, Deserialize)]
pub struct CategoryResponse {
    pub data: CategoryResponseData
}

#[derive(Debug, Deserialize)]
pub struct CategoryGroupWithCategories {
    pub id:         String,
    pub name:       String,
    pub hidden:     bool,
    pub deleted:    bool,
    pub categories: Vec<Category>
}

#[derive(Debug, Deserialize)]
pub struct CategoryGroup {
    pub id:         String,
    pub name:       String,
    pub hidden:     bool,
    pub deleted:    bool
}

#[derive(Debug, Deserialize)]
pub enum GoalType {
    TB,
    TBD,
    MF,
    NEED,
    DEBT
}

#[derive(Debug, Deserialize)]
pub struct Category {
    pub id:                         String,
    pub category_group_id:          String,
    pub category_group_name:        Option<String>,
    pub name:                       String,
    pub hidden:                     bool,
    pub original_category_group_id: Option<String>,
    pub note:                       Option<String>,
    pub budgeted:                   i64,
    pub activity:                   i64,
    pub balance:                    i64,
    pub goal_type:	                Option<GoalType>,
    pub goal_day:                   Option<i32>,
    pub goal_cadence:               Option<i32>,
    pub goal_cadence_frequency:     Option<i32>,
    pub goal_creation_month:        Option<String>,
    pub goal_target:                Option<i64>,
    pub goal_target_month:          Option<String>,
    pub goal_percentage_complete:	Option<i32>,
    pub goal_months_to_budget:	    Option<i32>,
    pub goal_under_funded:	        Option<i64>,
    pub goal_overall_funded:	    Option<i64>,
    pub goal_overall_left:	        Option<i64>,
    pub deleted:                    bool
}

#[derive(Debug, Deserialize)]
pub struct SaveCategoryResponseData {
    pub category:           Category,
    pub server_knowledge:   i64
}

#[derive(Debug, Deserialize)]
pub struct SaveCategoryResponse {
    pub data: SaveCategoryResponseData
}

#[derive(Debug, Deserialize)]
pub struct PayeesResponseData {
    pub payees:             Vec<Payee>,
    pub server_knowledge:   i64
}

#[derive(Debug, Deserialize)]
pub struct PayeesResponse {
    pub data: PayeesResponseData
}

#[derive(Debug, Deserialize)]
pub struct PayeeResponseData {
    pub payee: Payee
}

#[derive(Debug, Deserialize)]
pub struct PayeeResponse {
    pub data: PayeeResponseData
}

#[derive(Debug, Deserialize)]
pub struct Payee {
    pub id:                     String,
    pub name:                   String,
    pub transfer_account_id:    Option<String>,
    pub deleted:                bool
}

#[derive(Debug, Deserialize)]
pub struct PayeeLocationsResponseData {    
    pub payee_locations: Vec<PayeeLocation>
}

#[derive(Debug, Deserialize)]
pub struct PayeeLocationsResponse {
    pub data: PayeeLocationsResponseData
}

#[derive(Debug, Deserialize)]
pub struct PayeeLocationResponseData {
    pub payee_location: PayeeLocation
}

#[derive(Debug, Deserialize)]
pub struct PayeeLocationResponse {
    pub data: PayeeLocationResponseData
}

#[derive(Debug, Deserialize)]
pub struct PayeeLocation {
    pub id:         String,
    pub payee_id:   String,
    pub latitude:   String,
    pub longitude:  String,
    pub deleted:    bool
}

#[derive(Debug, Deserialize)]
pub struct TransactionsResponseData {
    pub transactions:       Vec<TransactionDetail>,
    pub server_knowledge:   i64
}

#[derive(Debug, Deserialize)]
pub struct TransactionsResponse {
    pub data: TransactionsResponseData
}

#[derive(Debug, Deserialize)]
pub struct HybridTransactionsResponseData {
    pub transactions: Vec<HybridTransaction>,
    pub server_knowledge: Option<i64>
}

#[derive(Debug, Deserialize)]
pub struct HybridTransactionsResponse {
    pub data: HybridTransactionsResponseData
}

#[derive(Debug, Deserialize)]
pub struct PutTransactionWrapper {
    pub transaction: SaveTransaction
}

#[derive(Debug, Deserialize)]
pub struct PostTransactionsWrapper {
    pub transaction:    Option<SaveTransaction>,
    pub transactions:   Option<Vec<SaveTransaction>>
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ClearedStatus {
    Cleared,
    Uncleared,
    Reconciled
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum FlagColor {
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    Purple
}

#[derive(Debug, Deserialize)]
pub struct SaveTransaction {
    pub account_id:         String,
    pub date:               String,
    pub amount:             i64,
    pub payee_id:           Option<String>,
    pub payee_name:         Option<String>,
    pub category_id:        Option<String>,
    pub memo:               Option<String>,
    pub cleared:            Option<ClearedStatus>,
    pub approved:           Option<bool>,
    pub flag_color:         Option<FlagColor>,
    pub import_id:          Option<String>,
    pub subtransactions:    Option<Vec<SaveSubTransaction>>
}

#[derive(Debug, Deserialize)]
pub struct PatchTransactionsWrapper {
    pub transactions: Vec<SaveTransactionWithId>
}

#[derive(Debug, Deserialize)]
pub struct SaveTransactionWithId {
    pub id:                 Option<String>,
    pub account_id:         Option<String>,
    pub date:               Option<String>,
    pub amount:             Option<i64>,
    pub payee_id:           Option<String>,
    pub payee_name:         Option<String>,
    pub category_id:        Option<String>,
    pub memo:               Option<String>,
    pub cleared:            Option<ClearedStatus>,
    pub approved:           Option<bool>,
    pub flag_color:         Option<FlagColor>,
    pub import_id:          Option<String>,
    pub subtransactions:    Option<Vec<SaveSubTransaction>>
}

#[derive(Debug, Deserialize)]
pub struct SaveTransactionWithOptionalFields {
    pub account_id:         Option<String>,
    pub date:               Option<String>,
    pub amount:             Option<i64>,
    pub payee_id:           Option<String>,
    pub payee_name:         Option<String>,
    pub category_id:        Option<String>,
    pub memo:               Option<String>,
    pub cleared:            Option<ClearedStatus>,
    pub approved:           Option<bool>,
    pub flag_color:         Option<FlagColor>,
    pub import_id:          Option<String>,
    pub subtransactions:    Option<Vec<SaveSubTransaction>>
}

#[derive(Debug, Deserialize)]
pub struct SaveSubTransaction {
    pub amount:         i64,
    pub payee_id:       Option<String>,
    pub payee_name:     Option<String>,
    pub category_id:    Option<String>,
    pub memo:           Option<String>
}

#[derive(Debug, Deserialize)]
pub struct SaveTransactionsResponseData {
    pub transaction_ids:        Vec<String>,
    pub transaction:            Option<TransactionDetail>,
    pub transactions:           Option<Vec<TransactionDetail>>,
    pub duplicate_import_ids:   Option<Vec<String>>,
    pub server_knowledge:       i64
}

#[derive(Debug, Deserialize)]
pub struct SaveTransactionsResponse {
    pub data: SaveTransactionsResponseData
}

#[derive(Debug, Deserialize)]
pub struct TransactionResponseData {
    pub transaction: TransactionDetail
}

#[derive(Debug, Deserialize)]
pub struct TransactionResponse {
    pub data: TransactionResponseData
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum DebtTransactionType {
    Payment,
    Refund,
    Fee,
    Interest,
    Escrow,
    BalancedAdjustment,
    Credit,
    Charge
}

#[derive(Debug, Deserialize)]
pub struct TransactionSummary {
    pub id:                         String,
    pub date:                       String,
    pub amount:                     i64,
    pub memo:                       Option<String>,
    pub cleared:                    ClearedStatus,
    pub approved:                   bool,
    pub flag_color:                 Option<FlagColor>,
    pub account_id:                 String,
    pub payee_id:                   Option<String>,
    pub category_id:                Option<String>,
    pub transfer_account_id:        Option<String>,
    pub transfer_transaction_id:    Option<String>,
    pub matched_transaction_id:     Option<String>,
    pub import_id:                  Option<String>,
    pub import_payee_name:          Option<String>,
    pub import_payee_name_original: Option<String>,
    pub debt_transaction_type:      Option<DebtTransactionType>,
    pub deleted:                    bool
}

#[derive(Debug, Deserialize)]
pub struct TransactionDetail {
    pub id:                         String,
    pub date:                       String,
    pub amount:                     i64,
    pub memo:                       Option<String>,
    pub cleared:                    ClearedStatus,
    pub approved:                   bool,
    pub flag_color:                 Option<FlagColor>,
    pub account_id:                 String,
    pub payee_id:                   Option<String>,
    pub category_id:                Option<String>,
    pub transfer_account_id:        Option<String>,
    pub matched_transaction_id:     Option<String>,
    pub import_id:                  Option<String>,
    pub import_payee_name:          Option<String>,
    pub import_payee_name_original: Option<String>,
    pub debt_transaction_type:      Option<DebtTransactionType>,
    pub deleted:                    bool,
    pub account_name:               String,
    pub payee_name:                 Option<String>,
    pub category_name:              Option<String>,
    pub subtransactions:            Vec<SubTransaction>
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TransactionType {
    Transaction,
    SubTransaction
}

#[derive(Debug, Deserialize)]
pub struct HybridTransaction {
    pub id:                         String,
    pub date:                       String,
    pub amount:                     i64,
    pub memo:                       Option<String>,
    pub cleared:                    ClearedStatus,
    pub approved:                   bool,
    pub flag_color:                 Option<FlagColor>,
    pub account_id:                 String,
    pub payee_id:                   Option<String>,
    pub category_id:                Option<String>,
    pub transfer_account_id:        Option<String>,
    pub matched_transaction_id:     Option<String>,
    pub import_id:                  Option<String>,
    pub import_payee_name:          Option<String>,
    pub import_payee_name_original: Option<String>,
    pub debt_transaction_type:      Option<DebtTransactionType>,
    pub deleted:                    bool,
    pub r#type:                     TransactionType,
    pub parent_transaction_id:      Option<String>,
    pub account_name:               String,
    pub payee_name:                 Option<String>,
    pub category_name:              Option<String>
}

#[derive(Debug, Deserialize)]
pub struct PatchMonthCategoryWrapper {
    pub category: SaveMonthCategory
}

#[derive(Debug, Deserialize)]
pub struct SaveMonthCategory {
    pub budgeted: i64
}

#[derive(Debug, Deserialize)]
pub struct TransactionsImportResponseData {
    pub transaction_ids: Vec<String>
}

#[derive(Debug, Deserialize)]
pub struct TransactionsImportResponse {
    pub data: TransactionsImportResponseData
}

#[derive(Debug, Deserialize)]
pub struct BulkResponseDataBulk {
    pub transaction_ids:        Vec<String>,
    pub duplicate_import_ids:   Vec<String>
}

#[derive(Debug, Deserialize)]
pub struct BulkResponseData {
    pub bulk: BulkResponseDataBulk
}

#[derive(Debug, Deserialize)]
pub struct BulkResponse {
    pub data: BulkResponseData
}

#[derive(Debug, Deserialize)]
pub struct BulkTransactions {
    pub transactions: Vec<SaveTransaction>
}

#[derive(Debug, Deserialize)]
pub struct SubTransaction {
    pub id:                         String,
    pub transaction_id:             String,
    pub amount:                     i64,
    pub memo:                       Option<String>,
    pub payee_id:                   Option<String>,
    pub payee_name:                 Option<String>,
    pub category_id:                Option<String>,
    pub category_name:              Option<String>,
    pub transfer_account_id:        Option<String>,
    pub transfer_transaction_id:    Option<String>,
    pub deleted:                    bool
}

#[derive(Debug, Deserialize)]
pub struct ScheduledTransactionsResponseData {
    pub scheduled_transactions: Vec<ScheduledTransactionDetail>,
    pub server_knowledge:       i64
}

#[derive(Debug, Deserialize)]
pub struct ScheduledTransactionsResponse {
    pub data: ScheduledTransactionsResponseData
}

#[derive(Debug, Deserialize)]
pub struct ScheduledTransactionResponseData {
    pub scheduled_transaction: ScheduledTransactionDetail
}

#[derive(Debug, Deserialize)]
pub struct ScheduledTransactionResponse {
    pub data: ScheduledTransactionResponseData
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ScheduleFrequency {
    Never,
    Daily,
    Weekly,
    EveryOtherWeek,
    TwiceAMonth,
    Every4Weeks,
    Monthly,
    EveryOtherMonth,
    Every3Months,
    Every4Months,
    TwiceAYear,
    Yearly,
    EveryOtherYear
}

#[derive(Debug, Deserialize)]
pub struct ScheduledTransactionSummary {
    pub id:                     String,
    pub date_first:             String,
    pub date_next:              String,
    pub frequency:              ScheduleFrequency,
    pub amount:                 i64,
    pub memo:                   Option<String>,
    pub flag_color:             Option<FlagColor>,
    pub account_id:             String,
    pub payee_id:               Option<String>,
    pub category_id:            Option<String>,
    pub transfer_account_id:    Option<String>,
    pub deleted:                bool
}

#[derive(Debug, Deserialize)]
pub struct ScheduledTransactionDetail {
    pub id:                     String,
    pub date_first:             String,
    pub date_next:              String,
    pub frequency:              ScheduleFrequency,
    pub amount:                 i64,
    pub memo:                   Option<String>,
    pub flag_color:             Option<FlagColor>,
    pub account_id:             String,
    pub payee_id:               Option<String>,
    pub category_id:            Option<String>,
    pub transfer_account_id:    Option<String>,
    pub deleted:                bool,
    pub account_name:           String,
    pub payee_name:             Option<String>,
    pub category_name:          Option<String>,
    pub subtransactions:        Vec<ScheduledSubTransaction>
}

#[derive(Debug, Deserialize)]
pub struct ScheduledSubTransaction {
    pub id:                         String,
    pub scheduled_transaction_id:   String,
    pub amount:                     i64,
    pub memo:                       Option<String>,
    pub payee_id:                   Option<String>,
    pub category_id:                Option<String>,
    pub transfer_account_id:        Option<String>,
    pub deleted:                    bool
}

#[derive(Debug, Deserialize)]
pub struct MonthSummariesResponseData {
    pub months:             Vec<MonthSummary>,
    pub server_knowledge:   i64
}

#[derive(Debug, Deserialize)]
pub struct MonthSummariesResponse {
    pub data: MonthSummariesResponseData
}

#[derive(Debug, Deserialize)]
pub struct MonthDetailResponseData {
    pub month: MonthDetail
}

#[derive(Debug, Deserialize)]
pub struct MonthDetailResponse {
    pub data: MonthDetailResponseData
}

#[derive(Debug, Deserialize)]
pub struct MonthSummary {
    pub month:          String,
    pub note:           Option<String>,
    pub income:         i64,
    pub budgeted:       i64,
    pub activity:       i64,
    pub to_be_budgeted: i64,
    pub age_of_money:   Option<i32>,
    pub deleted:        bool
}

#[derive(Debug, Deserialize)]
pub struct MonthDetail {
    pub month:          String,
    pub note:           Option<String>,
    pub income:         i64,
    pub budgeted:       i64,
    pub activity:       i64,
    pub to_be_budgeted: i64,
    pub age_of_money:   Option<i32>,
    pub deleted:        bool,
    pub categories:     Vec<Category>
}