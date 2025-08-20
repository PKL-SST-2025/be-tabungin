use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use validator::Validate;
use bigdecimal::BigDecimal;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub full_name: String,
    pub email: String,
    pub password_hash: String,
    pub avatar: Option<String>,
    pub is_admin: bool,
    pub nomor_telepon: Option<String>,
    pub alamat: Option<String>,
    pub posisi_jabatan: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub full_name: String,
    pub email: String,
    pub avatar: Option<String>,
    pub is_admin: bool,
    pub nomor_telepon: Option<String>,
    pub alamat: Option<String>,
    pub posisi_jabatan: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Testimoni {
    pub id: Uuid,
    pub user_id: Uuid,
    pub content: String,
    pub rating: i32,
    pub is_approved: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TestimoniWithUser {
    pub id: Uuid,
    pub content: String,
    pub rating: i32,
    pub is_approved: bool,
    pub created_at: DateTime<Utc>,
    pub user: UserResponse,
}

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(length(min = 2, max = 100, message = "Full name must be between 2 and 100 characters"))]
    pub full_name: String,
    
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
    
    #[validate(length(min = 6, message = "Password must be at least 6 characters"))]
    pub password: String,
    
    pub confirm_password: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
    
    #[validate(length(min = 1, message = "Password is required"))]
    pub password: String,
    
    pub is_admin: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: UserResponse,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateTestimoniRequest {
    #[validate(length(min = 10, max = 1000, message = "Content must be between 10 and 1000 characters"))]
    pub content: String,
    
    #[validate(range(min = 1, max = 5, message = "Rating must be between 1 and 5"))]
    pub rating: i32,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTestimoniRequest {
    pub content: Option<String>,
    pub rating: Option<i32>,
    pub is_approved: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserRequest {
    pub full_name: Option<String>,
    pub avatar: Option<String>,
    pub nomor_telepon: Option<String>,
    pub alamat: Option<String>,
    pub posisi_jabatan: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct ForgotPasswordRequest {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct ResetPasswordRequest {
    #[validate(length(min = 6, message = "Password must be at least 6 characters"))]
    pub new_password: String,
    
    pub confirm_password: String,
    pub reset_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct SavingsTarget {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub target_amount: BigDecimal,
    pub current_amount: Option<BigDecimal>,
    pub icon: Option<String>,
    pub icon_color: Option<String>,
    pub target_date: Option<chrono::NaiveDate>,
    pub is_completed: Option<bool>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Activity {
    pub id: Uuid,
    pub user_id: Uuid,
    pub savings_target_id: Option<Uuid>,
    pub activity_type: String,
    pub title: String,
    pub description: Option<String>,
    pub amount: Option<BigDecimal>,
    pub icon: Option<String>,
    pub icon_color: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UserStatistics {
    pub id: Uuid,
    pub user_id: Uuid,
    pub total_saved: Option<BigDecimal>,
    pub streak_days: Option<i32>,
    pub daily_average: Option<BigDecimal>,
    pub achievements_count: Option<i32>,
    pub last_deposit_date: Option<chrono::NaiveDate>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Achievement {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub description: String,
    pub icon: String,
    pub icon_color: String,
    pub earned_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateSavingsTargetRequest {
    #[validate(length(min = 1, max = 255, message = "Target name must be between 1 and 255 characters"))]
    pub name: String,
    
    #[validate(range(min = 1.0, message = "Target amount must be greater than 0"))]
    pub target_amount: f64,
    
    pub icon: Option<String>,
    pub icon_color: Option<String>,
    pub target_date: Option<chrono::NaiveDate>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateSavingsTargetRequest {
    pub name: Option<String>,
    pub target_amount: Option<f64>,
    pub current_amount: Option<f64>,
    pub icon: Option<String>,
    pub icon_color: Option<String>,
    pub target_date: Option<chrono::NaiveDate>,
    pub is_completed: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateActivityRequest {
    pub savings_target_id: Option<Uuid>,
    
    #[validate(length(min = 1, max = 50, message = "Activity type must be between 1 and 50 characters"))]
    pub activity_type: String,
    
    #[validate(length(min = 1, max = 255, message = "Title must be between 1 and 255 characters"))]
    pub title: String,
    
    pub description: Option<String>,
    pub amount: Option<f64>,
    pub icon: Option<String>,
    pub icon_color: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SavingsTargetResponse {
    pub id: Uuid,
    pub name: String,
    pub target_amount: f64,
    pub current_amount: f64,
    pub percentage: f64,
    pub icon: String,
    pub icon_color: String,
    pub target_date: Option<chrono::NaiveDate>,
    pub is_completed: bool,
    pub days_remaining: Option<i64>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActivityResponse {
    pub id: Uuid,
    pub activity_type: String,
    pub title: String,
    pub description: Option<String>,
    pub amount: f64,
    pub icon: String,
    pub icon_color: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Reminder {
    pub id: Uuid,
    pub user_id: Uuid,
    pub savings_target_id: Uuid,
    pub reminder_date: chrono::NaiveDate,
    pub reminder_type: String,
    pub title: String,
    pub description: Option<String>,
    pub is_completed: Option<bool>,
    pub is_notified: Option<bool>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReminderResponse {
    pub id: Uuid,
    pub reminder_date: chrono::NaiveDate,
    pub reminder_type: String,
    pub title: String,
    pub description: String,
    pub is_completed: bool,
    pub is_notified: bool,
    pub target_name: String,
    pub target_icon: String,
    pub target_icon_color: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Notification {
    pub id: i32,
    pub user_id: Uuid,
    pub r#type: String,
    pub message: String,
    pub read: bool,
    pub timestamp: chrono::NaiveDateTime,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            full_name: user.full_name,
            email: user.email,
            avatar: user.avatar,
            is_admin: user.is_admin,
            nomor_telepon: user.nomor_telepon,
            alamat: user.alamat,
            posisi_jabatan: user.posisi_jabatan,
            created_at: user.created_at,
        }
    }
}

impl From<SavingsTarget> for SavingsTargetResponse {
    fn from(target: SavingsTarget) -> Self {
        let target_amount_f64 = target.target_amount.to_string().parse().unwrap_or(0.0);
        let current_amount_f64 = target.current_amount
            .unwrap_or_else(|| bigdecimal::BigDecimal::from(0))
            .to_string()
            .parse()
            .unwrap_or(0.0);
        let percentage = if target_amount_f64 > 0.0 {
            (current_amount_f64 / target_amount_f64) * 100.0
        } else {
            0.0
        };
        
        let days_remaining = target.target_date.map(|date| {
            let today = chrono::Utc::now().date_naive();
            (date - today).num_days()
        });

        Self {
            id: target.id,
            name: target.name,
            target_amount: target_amount_f64,
            current_amount: current_amount_f64,
            percentage,
            icon: target.icon.unwrap_or_else(|| "💰".to_string()),
            icon_color: target.icon_color.unwrap_or_else(|| "bg-blue-500".to_string()),
            target_date: target.target_date,
            is_completed: target.is_completed.unwrap_or(false),
            days_remaining,
            created_at: target.created_at.unwrap_or_else(|| chrono::Utc::now()),
        }
    }
}

impl From<Activity> for ActivityResponse {
    fn from(activity: Activity) -> Self {
        let amount_f64 = activity.amount
            .unwrap_or_else(|| bigdecimal::BigDecimal::from(0))
            .to_string()
            .parse()
            .unwrap_or(0.0);
        
        Self {
            id: activity.id,
            activity_type: activity.activity_type,
            title: activity.title,
            description: activity.description,
            amount: amount_f64,
            icon: activity.icon.unwrap_or_else(|| "💰".to_string()),
            icon_color: activity.icon_color.unwrap_or_else(|| "bg-blue-500".to_string()),
            created_at: activity.created_at.unwrap_or_else(|| chrono::Utc::now()),
        }
    }
}
