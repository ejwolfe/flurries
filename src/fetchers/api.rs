use std::collections::HashMap;

pub struct Api {
    url: String,
    context: String,
    default_parameters: HashMap<String, String>,
}

impl Api {
    pub fn new(url: String, context: String, default_parameters: HashMap<String, String>) -> Self {
        Api {
            url,
            context,
            default_parameters,
        }
    }
    pub fn construct_request_url(&self, query_parameters: HashMap<String, String>) -> String {
        let mut request_url = self.url.clone();
        request_url.push_str(&self.context);
        let parameters = self.build_query_parameters(query_parameters);
        request_url.push_str(&parameters);

        request_url
    }

    fn build_query_parameters(&self, query_parameters: HashMap<String, String>) -> String {
        let mut parameters: String = String::from("?");
        let parameter_map: HashMap<String, String> = query_parameters
            .into_iter()
            .chain(self.default_parameters.clone())
            .collect();
        let parameter_amount = parameter_map.len();
        for (index, (key, value)) in parameter_map.iter().enumerate() {
            let mut parameter = key.clone();
            parameter.push_str("=");
            parameter.push_str(&value);
            parameters.push_str(&parameter);
            if index != parameter_amount - 1 {
                parameters.push_str("&");
            }
        }
        parameters
    }
}
