use crate::{
    metric_collector::MetricCollector,
    mqtt_config::MqttConfig,
    mqtt_wrapper::{MqttWrapper, QoS},
    protos::hoymiles::RealData::HMSStateResponse,
};

use chrono::prelude::DateTime;
use chrono::Local;
use log::{debug, warn};
use std::time::{Duration, UNIX_EPOCH};

pub struct SimpleMqtt<MQTT: MqttWrapper> {
    client: MQTT,
}

impl<MQTT: MqttWrapper> SimpleMqtt<MQTT> {
    pub fn new(config: &MqttConfig) -> Self {
        let client = MQTT::new(config, "-sm");
        Self { client }
    }
}

impl<MQTT: MqttWrapper> MetricCollector for SimpleMqtt<MQTT> {
    fn publish(&mut self, hms_state: &HMSStateResponse) {
        debug!("{hms_state}");

        let d = UNIX_EPOCH + Duration::from_secs(hms_state.time.max(0) as u64);
        let datetime = DateTime::<Local>::from(d);
        let inverter_local_time = datetime.format("%Y-%m-%d %H:%M:%S.%f").to_string();

        let mut topic_payload_pairs = vec![
            (
                "hms800wt2/inverter_local_time".to_string(),
                inverter_local_time,
            ),
            (
                "hms800wt2/pv_current_power".to_string(),
                (hms_state.pv_current_power as f32 / 10.).to_string(),
            ),
            (
                "hms800wt2/pv_daily_yield".to_string(),
                hms_state.pv_daily_yield.to_string(),
            ),
        ];

        if let Some(inverter) = hms_state.inverter_state.first() {
            topic_payload_pairs.extend([
                (
                    "hms800wt2/pv_grid_voltage".to_string(),
                    (inverter.grid_voltage as f32 / 10.).to_string(),
                ),
                (
                    "hms800wt2/pv_grid_freq".to_string(),
                    (inverter.grid_freq as f32 / 100.).to_string(),
                ),
                (
                    "hms800wt2/pv_inv_temperature".to_string(),
                    (inverter.temperature as f32 / 10.).to_string(),
                ),
                (
                    "hms800wt2/pv_inv_current".to_string(),
                    (inverter.grid_current as f32 / 100.).to_string(),
                ),
                (
                    "hms800wt2/pv_inv_power_factor".to_string(),
                    (inverter.power_factor as f32 / 1000.).to_string(),
                ),
            ]);
        }

        for port in &hms_state.port_state {
            let prefix = format!("hms800wt2/pv_port{}", port.pv_port);
            topic_payload_pairs.extend([
                (
                    format!("{prefix}_voltage"),
                    (port.pv_vol as f32 / 10.).to_string(),
                ),
                (
                    format!("{prefix}_curr"),
                    (port.pv_cur as f32 / 100.).to_string(),
                ),
                (
                    format!("{prefix}_power"),
                    (port.pv_power as f32 / 10.).to_string(),
                ),
                (
                    format!("{prefix}_energy"),
                    (port.pv_energy_total as f32).to_string(),
                ),
                (
                    format!("{prefix}_daily_yield"),
                    (port.pv_daily_yield as f32).to_string(),
                ),
            ]);
        }

        for (topic, payload) in topic_payload_pairs {
            if let Err(error) = self.client.publish(topic, QoS::AtMostOnce, true, payload) {
                warn!("mqtt error: {error:?}");
            }
        }
    }
}
