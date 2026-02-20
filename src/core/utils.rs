use chrono::{DateTime, Utc};

pub fn relative_time(ts: Option<i32>) -> String {
    let ts = match ts {
        Some(v) => v,
        None => return "-".into(),
    };

    let dt = DateTime::<Utc>::from_timestamp(ts.into(), 0);
    let dt = match dt {
        Some(v) => v,
        None => return "-".into(),
    };

    let diff = Utc::now() - dt;

    let mins = diff.num_minutes();
    if mins < 1 {
        return "just now".into();
    }
    if mins < 60 {
        return format!("{mins}m ago");
    }

    let hrs = diff.num_hours();
    if hrs < 24 {
        return format!("{hrs}h ago");
    }

    let days = diff.num_days();
    format!("{days}d ago")
}
