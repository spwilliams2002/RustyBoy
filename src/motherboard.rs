pub struct Motherboard {
    model: String,
    manufacturer: String,
    chipset: String,
    form_factor: String,
    socket_type: String,
}

impl Motherboard {
    // Constructor for creating a new Motherboard instance
    pub fn new(model: String, manufacturer: String, chipset: String, form_factor: String, socket_type: String) -> Self {
        Motherboard {
            model,
            manufacturer,
            chipset,
            form_factor,
            socket_type,
        }
    }

    // Getter for model
    pub fn get_model(&self) -> &String {
        &self.model
    }

    // Getter for manufacturer
    pub fn get_manufacturer(&self) -> &String {
        &self.manufacturer
    }

    // Setter for chipset
    pub fn set_chipset(&mut self, chipset: String) {
        self.chipset = chipset;
    }
}