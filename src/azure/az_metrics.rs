use rust_decimal::Decimal;



pub struct CustomMetric {
    metric: String,
    metric_namespace: String,
    dimension_names: Vec<String>,
    series: Vec<Series>,
}

impl CustomMetric {
    pub fn new(metric: String, metric_namespace: String) -> Self {
        CustomMetric {
            metric,
            metric_namespace,
            dimension_names: Vec::new(),
            series: Vec::new(),
        }
    }
}



pub struct Series {
    dimension_values: Vec<String>,
    min: Decimal,
    max: Decimal,
    sum: Decimal,
    count: i32,
}

impl Series {
    pub fn new() -> Self {
        Series {
            dimension_values: Vec::new(),
            min: Decimal::ZERO,
            max: Decimal::ZERO,
            sum: Decimal::ZERO,
            count: 0,
        }
    }
}

