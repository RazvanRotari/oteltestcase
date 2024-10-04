use opentelemetry_proto::tonic::{
    collector::logs::v1::ExportLogsServiceRequest,
    common::v1::{any_value::Value, AnyValue, ArrayValue, KeyValue},
    logs::v1::ResourceLogs,
    resource::v1::Resource,
};
fn main() {
    let input = ExportLogsServiceRequest {
        resource_logs: vec![ResourceLogs {
            resource: Some(Resource {
                attributes: vec![KeyValue {
                    key: "key1".to_string(),
                    value: Some(AnyValue {
                        value: Some(Value::ArrayValue(ArrayValue {
                            values: vec![
                                AnyValue {
                                    value: Some(Value::StringValue("item1".to_string())),
                                },
                                AnyValue {
                                    value: Some(Value::StringValue("item2".to_string())),
                                },
                            ],
                        })),
                    }),
                }],
                dropped_attributes_count: 0,
            }),
            schema_url: String::new(),
            scope_logs: Vec::new(),
        }],
    };
    println!("{}", serde_json::to_string_pretty(&input).unwrap());
}

// {
// "resourceLogs": [
//   {
//     "resource": {
//       "attributes": [
//         {
//           "key": "key1",
//           "value": {
//             "arrayValue": {
//               "values": [
//                 {
//                   "value": {
//                     "stringValue": "item1"
//                   }
//                 },
//                 {
//                   "value": {
//                     "stringValue": "item2"
//                   }
//                 }
//               ]
//             }
//           }
//         }
//       ],
//       "droppedAttributesCount": 0
//     },
//     "scopeLogs": [],
//     "schemaUrl": ""
//   }
// ]
// }
