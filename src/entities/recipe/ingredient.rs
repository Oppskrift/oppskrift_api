enum Unit {
    Gram,
    Litre,
    Piece,
}

pub struct Ingredient {
    name: String,
    quantity: f32,
    unit: Unit,
}
