#[derive(Debug, Serialize, Deserialize)]
pub struct GeoJson {
    #[serde(rename="type")]
    kind: String,
    features: Vec<Feature>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Feature {
   #[serde(rename="type")]
   kind: String,
   properties: Property,
   geometry: Geometry,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Property {
    y: f64,
    poly_id: i32,
    area: f64,
    malemetode: i32,
    oppr: i32,
    objtype: String,
    koordh: i32,
    h_malemeto: i32,
    max_avvik: i32,
    komm: i32,
    poly_: i32,
    x: f64,
    synbarhet: i32,
    noyaktighe: i32,
    navn: String,
    perimeter: f64,
    h_noyaktig: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Geometry {
    #[serde(rename="type")]
    kind: String,
    coordinates: Vec<Vec<Vec<f64>>>,
}

