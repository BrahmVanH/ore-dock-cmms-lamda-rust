use crate::models::{ prelude::* };

// Input types for future filtering
#[derive(InputObject)]
pub struct DashboardFilter {
    pub location_id: Option<String>,
    pub asset_type_id: Option<String>,
    pub manufacturer_id: Option<String>,
    pub date_range: Option<DateRangeFilter>,
}

#[derive(InputObject)]
pub struct DateRangeFilter {
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
}

// Output types
#[derive(Debug)]
pub struct DashboardStatistics {
    pub total_assets: i32,
    pub active_work_orders: i32,
    pub overdue_maintenances: i32,
    pub upcoming_maintenances: i32,
    pub assets_by_status: AssetsByStatus,
    pub work_orders_by_priority: WorkOrdersByPriority,
    pub work_orders_by_status: WorkOrdersByStatus,
    pub assets_down: i32,
    pub assets_needing_attention: i32,
    pub critical_work_orders: i32,
    pub total_maintenance_schedules: i32,
    pub active_maintenance_schedules: i32,
    pub assets_under_warranty: i32,
    pub recent_completions: i32,
}

#[Object]
impl DashboardStatistics {
    async fn total_assets(&self) -> i32 {
        self.total_assets
    }

    async fn active_work_orders(&self) -> i32 {
        self.active_work_orders
    }

    async fn overdue_maintenances(&self) -> i32 {
        self.overdue_maintenances
    }

    async fn upcoming_maintenances(&self) -> i32 {
        self.upcoming_maintenances
    }

    async fn assets_by_status(&self) -> &AssetsByStatus {
        &self.assets_by_status
    }

    async fn work_orders_by_priority(&self) -> &WorkOrdersByPriority {
        &self.work_orders_by_priority
    }

    async fn work_orders_by_status(&self) -> &WorkOrdersByStatus {
        &self.work_orders_by_status
    }

    async fn assets_down(&self) -> i32 {
        self.assets_down
    }

    async fn assets_needing_attention(&self) -> i32 {
        self.assets_needing_attention
    }

    async fn critical_work_orders(&self) -> i32 {
        self.critical_work_orders
    }

    async fn total_maintenance_schedules(&self) -> i32 {
        self.total_maintenance_schedules
    }

    async fn active_maintenance_schedules(&self) -> i32 {
        self.active_maintenance_schedules
    }

    async fn assets_under_warranty(&self) -> i32 {
        self.assets_under_warranty
    }

    async fn recent_completions(&self) -> i32 {
        self.recent_completions
    }

    // Computed fields
    async fn asset_operational_percentage(&self) -> f64 {
        if self.total_assets == 0 {
            0.0
        } else {
            ((self.assets_by_status.operational as f64) / (self.total_assets as f64)) * 100.0
        }
    }

    async fn maintenance_compliance_rate(&self) -> f64 {
        if self.total_maintenance_schedules == 0 {
            100.0
        } else {
            let compliant = self.total_maintenance_schedules - self.overdue_maintenances;
            ((compliant as f64) / (self.total_maintenance_schedules as f64)) * 100.0
        }
    }

    async fn work_order_completion_rate(&self) -> f64 {
        let total_work_orders =
            self.work_orders_by_status.open +
            self.work_orders_by_status.in_progress +
            self.work_orders_by_status.on_hold +
            self.work_orders_by_status.completed +
            self.work_orders_by_status.cancelled;

        if total_work_orders == 0 {
            0.0
        } else {
            ((self.work_orders_by_status.completed as f64) / (total_work_orders as f64)) * 100.0
        }
    }
}

#[derive(Debug)]
pub struct AssetsByStatus {
    pub operational: i32,
    pub down: i32,
    pub maintenance: i32,
    pub retired: i32,
    pub needs_attention: i32,
}

#[Object]
impl AssetsByStatus {
    async fn operational(&self) -> i32 {
        self.operational
    }

    async fn down(&self) -> i32 {
        self.down
    }

    async fn maintenance(&self) -> i32 {
        self.maintenance
    }

    async fn retired(&self) -> i32 {
        self.retired
    }

    async fn needs_attention(&self) -> i32 {
        self.needs_attention
    }
}

#[derive(Debug)]
pub struct WorkOrdersByPriority {
    pub low: i32,
    pub normal: i32,
    pub high: i32,
    pub urgent: i32,
    pub emergency: i32,
}

#[Object]
impl WorkOrdersByPriority {
    async fn low(&self) -> i32 {
        self.low
    }

    async fn normal(&self) -> i32 {
        self.normal
    }

    async fn high(&self) -> i32 {
        self.high
    }

    async fn urgent(&self) -> i32 {
        self.urgent
    }

    async fn emergency(&self) -> i32 {
        self.emergency
    }
}

#[derive(Debug)]
pub struct WorkOrdersByStatus {
    pub open: i32,
    pub in_progress: i32,
    pub on_hold: i32,
    pub completed: i32,
    pub cancelled: i32,
}

#[Object]
impl WorkOrdersByStatus {
    async fn open(&self) -> i32 {
        self.open
    }

    async fn in_progress(&self) -> i32 {
        self.in_progress
    }

    async fn on_hold(&self) -> i32 {
        self.on_hold
    }

    async fn completed(&self) -> i32 {
        self.completed
    }

    async fn cancelled(&self) -> i32 {
        self.cancelled
    }
}
