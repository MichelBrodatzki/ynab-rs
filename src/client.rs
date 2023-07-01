use either::{Either, Left, Right};

const API_BASE_URL: &str = "https://api.ynab.com/v1";

pub struct Client {
    api_token: String,
    client: reqwest::Client
}

impl Client {
    pub fn new(api_token: String) -> Client {
        let client: reqwest::Client = reqwest::Client::new();
        Client { api_token, client }
    }

    async fn get(&self, endpoint: &str) -> reqwest::Response {
        self.client
            .get(format!("{}{}", API_BASE_URL, endpoint))
            .header("Authorization", format!("Bearer {}", self.api_token))
            .send()
            .await
            .unwrap()
    }

    // User
    pub async fn get_user(&self) -> Either<crate::models::UserResponse, crate::models::ErrorResponse> {
        let raw_response: reqwest::Response = self.get("/user").await;

        if raw_response.status().is_success() {
            if let Ok(user_response) = raw_response.json::<crate::models::UserResponse>().await {
                return Left(user_response);
            } else {
                panic!("Returned data model is invalid for `get_user`.")
            }
        } else {
            if let Ok(error_response) = raw_response.json::<crate::models::ErrorResponse>().await {
                return Right(error_response);
            } else {
                panic!("Returned error model is invalid for `get_user`.")
            }
        }
    }

    // Budgets
    pub async fn get_budgets(&self, include_accounts: bool) -> Either<crate::models::BudgetSummaryResponse, crate::models::ErrorResponse> {
        let raw_response: reqwest::Response = self.get(if include_accounts { "/budgets?include_accounts=true" } else { "/budgets" }).await;

        if raw_response.status().is_success() {
            match raw_response.json::<crate::models::BudgetSummaryResponse>().await {
                Ok(budget_response) => return Left(budget_response),
                Err(error) => panic!("Returned data model is invalid for `get_budgets`. Error: {:?}", error)
            }
        } else {
            match raw_response.json::<crate::models::ErrorResponse>().await {
                Ok(error_response) => return Right(error_response),
                Err(error) => panic!("Returned error model is invalid for `get_budgets`. Error: {:?}", error)
            }
        }
    }

    pub async fn get_budget(&self, budget_id: &str, last_knowledge_of_server: Option<i64>) -> Either<crate::models::BudgetDetailResponse, crate::models::ErrorResponse> {
        let endpoint = match last_knowledge_of_server {
            Some(last_knowledge) => format!("/budgets/{}?last_knowledge_of_server={}", budget_id, last_knowledge),
            None => format!("/budgets/{}", budget_id)
        };

        let raw_response: reqwest::Response = self.get(endpoint.as_str()).await;

        if raw_response.status().is_success() {
            match raw_response.json::<crate::models::BudgetDetailResponse>().await {
                Ok(budget_response) => return Left(budget_response),
                Err(error) => panic!("Returned data model is invalid for `get_budget`. Error: {:?}", error)
            }
        } else {
            match raw_response.json::<crate::models::ErrorResponse>().await {
                Ok(error_response) => return Right(error_response),
                Err(error) => panic!("Returned error model is invalid for `get_budget`. Error: {:?}", error)
            }
        }
    }

    pub async fn get_budget_settings(&self, budget_id: &str) -> Either<crate::models::BudgetSettingsResponse, crate::models::ErrorResponse> {
        let endpoint = format!("/budgets/{}/settings", budget_id);

        let raw_response: reqwest::Response = self.get(endpoint.as_str()).await;

        if raw_response.status().is_success() {
            match raw_response.json::<crate::models::BudgetSettingsResponse>().await {
                Ok(settings_response) => return Left(settings_response),
                Err(error) => panic!("Returned data model is invalid for `get_budget_settings`. Error: {:?}", error)
            }
        } else {
            match raw_response.json::<crate::models::ErrorResponse>().await {
                Ok(error_response) => return Right(error_response),
                Err(error) => panic!("Returned error model is invalid for `get_budget_settings`. Error: {:?}", error)
            }
        }
    }

    // Accounts
    pub async fn get_account_list(&self, budget_id: &str, last_knowledge_of_server: Option<i64>) -> Either<crate::models::AccountsResponse, crate::models::ErrorResponse> {
        let endpoint = match last_knowledge_of_server {
            Some(last_knowledge) => format!("/budgets/{}/accounts?last_knowledge_of_server={}", budget_id, last_knowledge),
            None => format!("/budgets/{}/accounts", budget_id)
        };

        let raw_response: reqwest::Response = self.get(endpoint.as_str()).await;

        if raw_response.status().is_success() {
            match raw_response.json::<crate::models::AccountsResponse>().await {
                Ok(accounts_response) => return Left(accounts_response),
                Err(error) => panic!("Returned data model is invalid for `get_account_list`. Error: {:?}", error)
            }
        } else {
            match raw_response.json::<crate::models::ErrorResponse>().await {
                Ok(error_response) => return Right(error_response),
                Err(error) => panic!("Returned error model is invalid for `get_account_list`. Error: {:?}", error)
            }
        }
    }

    pub async fn get_account(&self, budget_id: &str, account_id: &str) -> Either<crate::models::AccountResponse, crate::models::ErrorResponse> {
        let endpoint = format!("/budgets/{}/accounts/{}", budget_id, account_id);

        let raw_response: reqwest::Response = self.get(endpoint.as_str()).await;

        if raw_response.status().is_success() {
            match raw_response.json::<crate::models::AccountResponse>().await {
                Ok(account_response) => return Left(account_response),
                Err(error) => panic!("Returned data model is invalid for `get_account`. Error: {:?}", error)
            }
        } else {
            match raw_response.json::<crate::models::ErrorResponse>().await {
                Ok(error_response) => return Right(error_response),
                Err(error) => panic!("Returned error model is invalid for `get_account`. Error: {:?}", error)
            }
        }
    }

    // Categories
    pub async fn get_category_list(&self, budget_id: &str, last_knowledge_of_server: Option<i64>) -> Either<crate::models::CategoriesResponse, crate::models::ErrorResponse> {
        let endpoint = match last_knowledge_of_server {
            Some(last_knowledge) => format!("/budgets/{}/categories?last_knowledge_of_server={}", budget_id, last_knowledge),
            None => format!("/budgets/{}/categories", budget_id)
        };

        let raw_response: reqwest::Response = self.get(endpoint.as_str()).await;

        if raw_response.status().is_success() {
            match raw_response.json::<crate::models::CategoriesResponse>().await {
                Ok(categories_response) => return Left(categories_response),
                Err(error) => panic!("Returned data model is invalid for `get_category_list`. Error: {:?}", error)
            }
        } else {
            match raw_response.json::<crate::models::ErrorResponse>().await {
                Ok(error_response) => return Right(error_response),
                Err(error) => panic!("Returned error model is invalid for `get_category_list`. Error: {:?}", error)
            }
        }
    }

    pub async fn get_category(&self, budget_id: &str, category_id: &str) -> Either<crate::models::CategoryResponse, crate::models::ErrorResponse> {
        let endpoint = format!("/budgets/{}/categories/{}", budget_id, category_id);

        let raw_response: reqwest::Response = self.get(endpoint.as_str()).await;

        if raw_response.status().is_success() {
            match raw_response.json::<crate::models::CategoryResponse>().await {
                Ok(category_response) => return Left(category_response),
                Err(error) => panic!("Returned data model is invalid for `get_category`. Error: {:?}", error)
            }
        } else {
            match raw_response.json::<crate::models::ErrorResponse>().await {
                Ok(error_response) => return Right(error_response),
                Err(error) => panic!("Returned error model is invalid for `get_category`. Error: {:?}", error)
            }
        }
    }

    pub async fn get_category_for_month(&self, budget_id: &str, date: &str, category_id: &str) -> Either<crate::models::CategoryResponse, crate::models::ErrorResponse> {
        let endpoint = format!("/budgets/{}/months/{}/categories/{}", budget_id, date, category_id);

        let raw_response: reqwest::Response = self.get(endpoint.as_str()).await;

        if raw_response.status().is_success() {
            match raw_response.json::<crate::models::CategoryResponse>().await {
                Ok(category_response) => return Left(category_response),
                Err(error) => panic!("Returned data model is invalid for `get_category_for_month`. Error: {:?}", error)
            }
        } else {
            match raw_response.json::<crate::models::ErrorResponse>().await {
                Ok(error_response) => return Right(error_response),
                Err(error) => panic!("Returned error model is invalid for `get_category_for_month`. Error: {:?}", error)
            }
        }
    }

    // Payees
    pub async fn get_payee_list(&self, budget_id: &str, last_knowledge_of_server: Option<i64>) -> Either<crate::models::PayeesResponse, crate::models::ErrorResponse> {
        let endpoint = match last_knowledge_of_server {
            Some(last_knowledge) => format!("/budgets/{}/payees?last_knowledge_of_server={}", budget_id, last_knowledge),
            None => format!("/budgets/{}/payees", budget_id)
        };

        let raw_response: reqwest::Response = self.get(endpoint.as_str()).await;

        if raw_response.status().is_success() {
            match raw_response.json::<crate::models::PayeesResponse>().await {
                Ok(payees_response) => return Left(payees_response),
                Err(error) => panic!("Returned data model is invalid for `get_payee_list`. Error: {:?}", error)
            }
        } else {
            match raw_response.json::<crate::models::ErrorResponse>().await {
                Ok(error_response) => return Right(error_response),
                Err(error) => panic!("Returned error model is invalid for `get_payee_list`. Error: {:?}", error)
            }
        }
    }

    pub async fn get_payee(&self, budget_id: &str, payee_id: &str) -> Either<crate::models::PayeeResponse, crate::models::ErrorResponse> {
        let endpoint = format!("/budgets/{}/payees/{}", budget_id, payee_id);

        let raw_response: reqwest::Response = self.get(endpoint.as_str()).await;

        if raw_response.status().is_success() {
            match raw_response.json::<crate::models::PayeeResponse>().await {
                Ok(payee_response) => return Left(payee_response),
                Err(error) => panic!("Returned data model is invalid for `get_payee`. Error: {:?}", error)
            }
        } else {
            match raw_response.json::<crate::models::ErrorResponse>().await {
                Ok(error_response) => return Right(error_response),
                Err(error) => panic!("Returned error model is invalid for `get_payee`. Error: {:?}", error)
            }
        }
    }

    // Payee Locations
    pub async fn get_payee_location_list(&self, budget_id: &str) -> Either<crate::models::PayeeLocationsResponse, crate::models::ErrorResponse> {
        let endpoint = format!("/budgets/{}/payee_locations", budget_id);

        let raw_response: reqwest::Response = self.get(endpoint.as_str()).await;

        if raw_response.status().is_success() {
            match raw_response.json::<crate::models::PayeeLocationsResponse>().await {
                Ok(payee_location_response) => return Left(payee_location_response),
                Err(error) => panic!("Returned data model is invalid for `get_payee_location_list`. Error: {:?}", error)
            }
        } else {
            match raw_response.json::<crate::models::ErrorResponse>().await {
                Ok(error_response) => return Right(error_response),
                Err(error) => panic!("Returned error model is invalid for `get_payee_location_list`. Error: {:?}", error)
            }
        }
    }

    pub async fn get_payee_location(&self, budget_id: &str, payee_location_id: &str) -> Either<crate::models::PayeeLocationResponse, crate::models::ErrorResponse> {
        let endpoint = format!("/budgets/{}/payee_locations/{}", budget_id, payee_location_id);

        let raw_response: reqwest::Response = self.get(endpoint.as_str()).await;

        if raw_response.status().is_success() {
            match raw_response.json::<crate::models::PayeeLocationResponse>().await {
                Ok(payee_location_response) => return Left(payee_location_response),
                Err(error) => panic!("Returned data model is invalid for `get_payee_location`. Error: {:?}", error)
            }
        } else {
            match raw_response.json::<crate::models::ErrorResponse>().await {
                Ok(error_response) => return Right(error_response),
                Err(error) => panic!("Returned error model is invalid for `get_payee_location`. Error: {:?}", error)
            }
        }
    }

    pub async fn get_payee_locations_for_payee(&self, budget_id: &str, payee_id: &str) -> Either<crate::models::PayeeLocationsResponse, crate::models::ErrorResponse> {
        let endpoint = format!("/budgets/{}/payees/{}/payee_locations", budget_id, payee_id);

        let raw_response: reqwest::Response = self.get(endpoint.as_str()).await;

        if raw_response.status().is_success() {
            match raw_response.json::<crate::models::PayeeLocationsResponse>().await {
                Ok(payee_location_response) => return Left(payee_location_response),
                Err(error) => panic!("Returned data model is invalid for `get_payee_location_for_payee`. Error: {:?}", error)
            }
        } else {
            match raw_response.json::<crate::models::ErrorResponse>().await {
                Ok(error_response) => return Right(error_response),
                Err(error) => panic!("Returned error model is invalid for `get_payee_location_for_payee`. Error: {:?}", error)
            }
        }
    }


    // Months
    pub async fn get_month_list(&self, budget_id: &str, last_knowledge_of_server: Option<i64>) -> Either<crate::models::MonthSummariesResponse, crate::models::ErrorResponse> {
        let endpoint = match last_knowledge_of_server {
            Some(last_knowledge) => format!("/budgets/{}/months?last_knowledge_of_server={}", budget_id, last_knowledge),
            None => format!("/budgets/{}/months", budget_id)
        };

        let raw_response: reqwest::Response = self.get(endpoint.as_str()).await;

        if raw_response.status().is_success() {
            match raw_response.json::<crate::models::MonthSummariesResponse>().await {
                Ok(months_response) => return Left(months_response),
                Err(error) => panic!("Returned data model is invalid for `get_month_list`. Error: {:?}", error)
            }
        } else {
            match raw_response.json::<crate::models::ErrorResponse>().await {
                Ok(error_response) => return Right(error_response),
                Err(error) => panic!("Returned error model is invalid for `get_month_list`. Error: {:?}", error)
            }
        }
    }

    pub async fn get_month(&self, budget_id: &str, date: &str) -> Either<crate::models::MonthDetailResponse, crate::models::ErrorResponse> {
        let endpoint = format!("/budgets/{}/months/{}", budget_id, date);

        let raw_response: reqwest::Response = self.get(endpoint.as_str()).await;

        if raw_response.status().is_success() {
            match raw_response.json::<crate::models::MonthDetailResponse>().await {
                Ok(month_response) => return Left(month_response),
                Err(error) => panic!("Returned data model is invalid for `get_month`. Error: {:?}", error)
            }
        } else {
            match raw_response.json::<crate::models::ErrorResponse>().await {
                Ok(error_response) => return Right(error_response),
                Err(error) => panic!("Returned error model is invalid for `get_month`. Error: {:?}", error)
            }
        }
    }

    // Transactions
    pub async fn get_transaction_list(&self, budget_id: &str, since_date: Option<&str>, transaction_type: Option<&str>, last_knowledge_of_server: Option<i64>) -> Either<crate::models::TransactionsResponse, crate::models::ErrorResponse> {
        let mut parameters: Vec<String> = vec![];

        if let Some(some_since_date) = since_date {
            parameters.push(format!("since_date={}", some_since_date));
        }

        if let Some(some_transaction_type) = transaction_type {
            parameters.push(format!("type={}", some_transaction_type));
        }

        if let Some(some_last_knowledge_of_server) = last_knowledge_of_server {
            parameters.push(format!("last_knowledge_of_server={}", some_last_knowledge_of_server));
        }

        let mut endpoint: String = format!("/budgets/{}/transactions", budget_id);
        if parameters.len() > 0 {
            endpoint = format!("/budgets/{}/transactions?{}", budget_id, parameters.join("&"));
        }

        let raw_response: reqwest::Response = self.get(endpoint.as_str()).await;

        if raw_response.status().is_success() {
            match raw_response.json::<crate::models::TransactionsResponse>().await {
                Ok(transactions_response) => return Left(transactions_response),
                Err(error) => panic!("Returned data model is invalid for `get_transaction_list`. Error: {:?}", error)
            }
        } else {
            match raw_response.json::<crate::models::ErrorResponse>().await {
                Ok(error_response) => return Right(error_response),
                Err(error) => panic!("Returned error model is invalid for `get_transaction_list`. Error: {:?}", error)
            }
        }
    }

    pub async fn get_transaction(&self, budget_id: &str, transaction_id: &str) -> Either<crate::models::TransactionResponse, crate::models::ErrorResponse> {
        let endpoint = format!("/budgets/{}/transactions/{}", budget_id, transaction_id);

        let raw_response: reqwest::Response = self.get(endpoint.as_str()).await;

        if raw_response.status().is_success() {
            match raw_response.json::<crate::models::TransactionResponse>().await {
                Ok(transaction_response) => return Left(transaction_response),
                Err(error) => panic!("Returned data model is invalid for `get_transaction`. Error: {:?}", error)
            }
        } else {
            match raw_response.json::<crate::models::ErrorResponse>().await {
                Ok(error_response) => return Right(error_response),
                Err(error) => panic!("Returned error model is invalid for `get_transaction`. Error: {:?}", error)
            }
        }
    }

    pub async fn get_transaction_list_for_account(&self, budget_id: &str, account_id: &str, since_date: Option<&str>, transaction_type: Option<&str>, last_knowledge_of_server: Option<i64>) -> Either<crate::models::TransactionsResponse, crate::models::ErrorResponse> {
        let mut parameters: Vec<String> = vec![];

        if let Some(some_since_date) = since_date {
            parameters.push(format!("since_date={}", some_since_date));
        }

        if let Some(some_transaction_type) = transaction_type {
            parameters.push(format!("type={}", some_transaction_type));
        }

        if let Some(some_last_knowledge_of_server) = last_knowledge_of_server {
            parameters.push(format!("last_knowledge_of_server={}", some_last_knowledge_of_server));
        }

        let mut endpoint: String = format!("/budgets/{}/accounts/{}/transactions", budget_id, account_id);
        if parameters.len() > 0 {
            endpoint = format!("/budgets/{}/accounts/{}/transactions?{}", budget_id, account_id, parameters.join("&"));
        }

        let raw_response: reqwest::Response = self.get(endpoint.as_str()).await;

        if raw_response.status().is_success() {
            match raw_response.json::<crate::models::TransactionsResponse>().await {
                Ok(transactions_response) => return Left(transactions_response),
                Err(error) => panic!("Returned data model is invalid for `get_transaction_list_for_account`. Error: {:?}", error)
            }
        } else {
            match raw_response.json::<crate::models::ErrorResponse>().await {
                Ok(error_response) => return Right(error_response),
                Err(error) => panic!("Returned error model is invalid for `get_transaction_list_for_account`. Error: {:?}", error)
            }
        }
    }

    pub async fn get_transaction_list_for_category(&self, budget_id: &str, category_id: &str, since_date: Option<&str>, transaction_type: Option<&str>, last_knowledge_of_server: Option<i64>) -> Either<crate::models::TransactionsResponse, crate::models::ErrorResponse> {
        let mut parameters: Vec<String> = vec![];

        if let Some(some_since_date) = since_date {
            parameters.push(format!("since_date={}", some_since_date));
        }

        if let Some(some_transaction_type) = transaction_type {
            parameters.push(format!("type={}", some_transaction_type));
        }

        if let Some(some_last_knowledge_of_server) = last_knowledge_of_server {
            parameters.push(format!("last_knowledge_of_server={}", some_last_knowledge_of_server));
        }

        let mut endpoint: String = format!("/budgets/{}/categories/{}/transactions", budget_id, category_id);
        if parameters.len() > 0 {
            endpoint = format!("/budgets/{}/categories/{}/transactions?{}", budget_id, category_id, parameters.join("&"));
        }

        let raw_response: reqwest::Response = self.get(endpoint.as_str()).await;

        if raw_response.status().is_success() {
            match raw_response.json::<crate::models::TransactionsResponse>().await {
                Ok(transactions_response) => return Left(transactions_response),
                Err(error) => panic!("Returned data model is invalid for `get_transaction_list_for_category`. Error: {:?}", error)
            }
        } else {
            match raw_response.json::<crate::models::ErrorResponse>().await {
                Ok(error_response) => return Right(error_response),
                Err(error) => panic!("Returned error model is invalid for `get_transaction_list_for_category`. Error: {:?}", error)
            }
        }
    }

    pub async fn get_transaction_list_for_payee(&self, budget_id: &str, payee_id: &str, since_date: Option<&str>, transaction_type: Option<&str>, last_knowledge_of_server: Option<i64>) -> Either<crate::models::HybridTransactionsResponse, crate::models::ErrorResponse> {
        let mut parameters: Vec<String> = vec![];

        if let Some(some_since_date) = since_date {
            parameters.push(format!("since_date={}", some_since_date));
        }

        if let Some(some_transaction_type) = transaction_type {
            parameters.push(format!("type={}", some_transaction_type));
        }

        if let Some(some_last_knowledge_of_server) = last_knowledge_of_server {
            parameters.push(format!("last_knowledge_of_server={}", some_last_knowledge_of_server));
        }

        let mut endpoint: String = format!("/budgets/{}/payees/{}/transactions", budget_id, payee_id);
        if parameters.len() > 0 {
            endpoint = format!("/budgets/{}/payees/{}/transactions?{}", budget_id, payee_id, parameters.join("&"));
        }

        let raw_response: reqwest::Response = self.get(endpoint.as_str()).await;

        if raw_response.status().is_success() {
            match raw_response.json::<crate::models::HybridTransactionsResponse>().await {
                Ok(transactions_response) => return Left(transactions_response),
                Err(error) => panic!("Returned data model is invalid for `get_transaction_list_for_payee`. Error: {:?}", error)
            }
        } else {
            match raw_response.json::<crate::models::ErrorResponse>().await {
                Ok(error_response) => return Right(error_response),
                Err(error) => panic!("Returned error model is invalid for `get_transaction_list_for_payee`. Error: {:?}", error)
            }
        }
    }

    // Scheduled Transactions
    pub async fn get_scheduled_transaction_list(&self, budget_id: &str, last_knowledge_of_server: Option<i64>) -> Either<crate::models::ScheduledTransactionsResponse, crate::models::ErrorResponse> {
        let endpoint = match last_knowledge_of_server {
            Some(last_knowledge) => format!("/budgets/{}/scheduled_transactions?last_knowledge_of_server={}", budget_id, last_knowledge),
            None => format!("/budgets/{}/scheduled_transactions", budget_id)
        };

        let raw_response: reqwest::Response = self.get(endpoint.as_str()).await;

        if raw_response.status().is_success() {
            match raw_response.json::<crate::models::ScheduledTransactionsResponse>().await {
                Ok(st_response) => return Left(st_response),
                Err(error) => panic!("Returned data model is invalid for `get_scheduled_transaction_list`. Error: {:?}", error)
            }
        } else {
            match raw_response.json::<crate::models::ErrorResponse>().await {
                Ok(error_response) => return Right(error_response),
                Err(error) => panic!("Returned error model is invalid for `get_scheduled_transaction_list`. Error: {:?}", error)
            }
        }
    }

    pub async fn get_scheduled_transaction(&self, budget_id: &str, scheduled_transaction_id: &str) -> Either<crate::models::ScheduledTransactionResponse, crate::models::ErrorResponse> {
        let endpoint = format!("/budgets/{}/scheduled_transactions/{}", budget_id, scheduled_transaction_id);

        let raw_response: reqwest::Response = self.get(endpoint.as_str()).await;

        if raw_response.status().is_success() {
            match raw_response.json::<crate::models::ScheduledTransactionResponse>().await {
                Ok(st_response) => return Left(st_response),
                Err(error) => panic!("Returned data model is invalid for `get_payee`. Error: {:?}", error)
            }
        } else {
            match raw_response.json::<crate::models::ErrorResponse>().await {
                Ok(error_response) => return Right(error_response),
                Err(error) => panic!("Returned error model is invalid for `get_payee`. Error: {:?}", error)
            }
        }
    }

    
}