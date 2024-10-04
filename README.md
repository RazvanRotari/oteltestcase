## Rust
```
 {
 "resourceLogs": [
   {
     "resource": {
       "attributes": [
         {
           "key": "key1",
           "value": {
             "arrayValue": {
               "values": [
                 {
                   "value": {
                     "stringValue": "item1"
                   }
                 },
                 {
                   "value": {
                     "stringValue": "item2"
                   }
                 }
               ]
             }
           }
         }
       ],
       "droppedAttributesCount": 0
     },
     "scopeLogs": [],
     "schemaUrl": ""
   }
 ]
 }
```

## JS

```
{
  "resourceSpans": [
    {
      "resource": {
        "attributes": [
          {
            "key": "service.name",
            "value": {
              "kvlistValue": {
                "values": [
                  {
                    "key": "test",
                    "value": {
                      "stringValue": "xml-http-web-service"
                    }
                  },
                  {
                    "key": "key2",
                    "value": {
                      "arrayValue": {
                        "values": [
                          {
                            "intValue": 1
                          },
                          {
                            "intValue": 2
                          },
                          {
                            "intValue": 3
                          }
                        ]
                      }
                    }
                  }
                ]
              }
            }
          },
          {
            "key": "telemetry.sdk.language",
            "value": {
              "stringValue": "webjs"
            }
          },
          {
            "key": "telemetry.sdk.name",
            "value": {
              "stringValue": "opentelemetry"
            }
          },
          {
            "key": "telemetry.sdk.version",
            "value": {
              "stringValue": "1.25.1"
            }
          }
        ],
        "droppedAttributesCount": 0
      },
      "scopeSpans": [
        {
          "scope": {
            "name": "@opentelemetry/instrumentation-xml-http-request",
            "version": "0.52.1"
          },
          "spans": [
            {
              "traceId": "642d8e894142a49009ae72fb3f16e811",
              "spanId": "c0d0cc19dc980004",
              "parentSpanId": "8750ffdab3cdda53",
              "name": "GET",
              "kind": 3,
              "startTimeUnixNano": "1728045280963000000",
              "endTimeUnixNano": "1728045285963000000",
              "attributes": [
                {
                  "key": "http.method",
                  "value": {
                    "stringValue": "GET"
                  }
                },
                {
                  "key": "http.url",
                  "value": {
                    "stringValue": "https://httpbin.org/get"
                  }
                },
                {
                  "key": "http.response_content_length",
                  "value": {
                    "intValue": 812
                  }
                },
                {
                  "key": "http.status_code",
                  "value": {
                    "intValue": 200
                  }
                },
                {
                  "key": "http.status_text",
                  "value": {
                    "stringValue": ""
                  }
                },
                {
                  "key": "http.host",
                  "value": {
                    "stringValue": "httpbin.org"
                  }
                },
                {
                  "key": "http.scheme",
                  "value": {
                    "stringValue": "https"
                  }
                },
                {
                  "key": "http.user_agent",
                  "value": {
                    "stringValue": "Mozilla/5.0 (X11; Linux x86_64; rv:131.0) Gecko/20100101 Firefox/131.0"
                  }
                }
              ],
              "droppedAttributesCount": 0,
              "events": [
                {
                  "attributes": [],
                  "name": "open",
                  "timeUnixNano": "1728045280963000000",
                  "droppedAttributesCount": 0
                },
                {
                  "attributes": [],
                  "name": "send",
                  "timeUnixNano": "1728045280963000000",
                  "droppedAttributesCount": 0
                },
                {
                  "attributes": [],
                  "name": "fetchStart",
                  "timeUnixNano": "1728045280964000000",
                  "droppedAttributesCount": 0
                },
                {
                  "attributes": [],
                  "name": "responseEnd",
                  "timeUnixNano": "1728045285961000000",
                  "droppedAttributesCount": 0
                },
                {
                  "attributes": [],
                  "name": "loaded",
                  "timeUnixNano": "1728045285963000000",
                  "droppedAttributesCount": 0
                }
              ],
              "droppedEventsCount": 0,
              "status": {
                "code": 0
              },
              "links": [],
              "droppedLinksCount": 0
            }
          ]
        }
      ]
    }
  ]
}
```
