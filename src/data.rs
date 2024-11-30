use crate::model::Product;
use crate::configuration::Settings;

pub fn fetch_products(_settings: &Settings) -> Vec<Product> {
    vec![
        Product {
            id: 1,
            name: "Samsung Smart TV".to_string(),
            price: 1199.99,
            description: "Immerse yourself in spectacular visuals with this Samsung LED Smart TV. It features 4K Ultra HD resolution and 60Hz refresh rate for crisp and clear picture quality with smooth visuals. Plus, 4K upscaling improves the quality of low-resolution content to almost 4K resolution. The Tizen OS provides user-friendly interface and more entertainment options.".to_string(),
            image: "/tv.jpg".to_string()
        },
        Product {
            id: 2,
            name: "HP Laptop".to_string(),
            price: 299.99,
            description: "Sleek and powerful, the HP 15.6-inch laptop can be your reliable computing companion anywhere. Equipped with Intel Core i3 N305 processor and 8GB RAM, this laptop enables smoother performance and multitasking. The 512GB solid state drive provides ample space for storing your files and media. The dual speakers offer enhanced audio experience.".to_string(),
            image: "/laptop.jpg".to_string()
        },
        Product {
            id: 3,
            name: "PlayStation 5".to_string(),
            price: 549.99,
            description: "Elevate your Fortnite gameplay with the PlayStation 5 Slim console. This Fortnite Cobalt Star bundle includes a PlayStation 5 console, eight in-game cosmetics (estimated value of 5,000 V-bucks), and 1,000 V-bucks. The Cobalt Star pack features an outfit, back bling, pickaxe, wrap, and more. Some cosmetics may also be available in the Fortnite Item Shop.".to_string(),
            image: "/playstation.jpg".to_string()
        },
        Product {
            id: 4,
            name: "Apple iPhone".to_string(),
            price: 1749.99,
            description: "The Apple iPhone 15 Pro Max features the groundbreaking A17 Pro chip, a customizable Action button, and the a powerful iPhone camera system. Forged in titanium, it boasts a strong and light aerospace-grade titanium design with a textured matte-glass back.".to_string(),
            image: "/phone.jpg".to_string()
        },
        Product {
            id: 5,
            name: "Cordless Stick Vacuum".to_string(),
            price: 799.99,
            description: "Enhance deep-cleaning sessions with the Dyson V15 Detect cordless stick vacuum. It's designed to intelligently adapt the suction power based on how dirty your floors are, allowing you to achieve a comprehensive hard floor or carpet clean with no guesswork. It has an advanced filter for better dust pickup and runs for up to 60 minutes on one charge.".to_string(),
            image: "/vacuum.jpg".to_string()
        },
        Product {
            id: 6,
            name: "Espresso".to_string(),
            price: 399.99,
            description: "Create expert coffee drinks at home with the Philips 800 automatic espresso machine. Simply add your favourite coffee, then use the My Coffee Choice menu to select your preferred aroma strength and beverage size. The attached milk frother allows you to complete your drink with silky smooth milk froth.".to_string(),
            image: "/milk.jpg".to_string()
        },
        Product {
            id: 7,
            name: "OLED smart tv".to_string(),
            price: 1999.99,
            description: "Experience the ultimate in visual brilliance with the LG 65 4K OLED Evo ThinQ AI smart TV. It comes with gaming features like NVIDIA G-Sync, AMD FreeSync Premium, and VRR to ensure enhanced gameplay. With a lightning-fast 0.1ms response time and a native 120Hz refresh rate, it delivers crisp and smooth imagery. Thanks to the intuitive AI features, accessing your favourite content is a breeze.".to_string(),
            image: "/OLED.jpg".to_string()
        },
        Product {
            id: 8,
            name: "Roku Smart TV".to_string(),
            price: 319.99,
            description: "Experience next level of visual entertainment with TCL 55 S-Series smart TV. Its 4K UHD screen resolution, 6.5ms response time and a 60Hz refresh rate guarantee extremely smooth, clear and realistic picture. Three HDMI 1.4a inputs let you watch high-quality content from different sources and dual-band Wi-Fi allows you to wirelessly stream various content.".to_string(),
            image: "/Roku.jpg".to_string()
        },
        Product {
            id: 9,
            name: "Bluetooth Headphones".to_string(),
            price: 279.99,
            description: "Get the personalised listening experience you deserve with the Sony WH-1000XM4 over-ear Bluetooth headphones. Adaptive noise cancellation with Dual Noise Sensor and powerful Edge-AI technology combine to bring you an intelligent, premium audio experience. The built-in Google Assistant and Amazon Alexa let you control your music hands-free with voice controls.".to_string(),
            image: "/headphones.jpg".to_string()
        },
        Product {
            id: 10,
            name: "Sound Bar".to_string(),
            price: 899.99,
            description: "Get the ultimate cinematic experience right at home with the JBL Bar 1000 880W 7.1.4 channel sound bar. With True Dolby Atmos, DTS:X and Multibeam Surround Sound technologies, it creates an immersive 3D soundscape while Pureview technology ensures voice clarity in dialogues. This sound bar comes with a 10 wireless subwoofer that gives you thundering bass in action scenes.".to_string(),
            image: "/bar.jpg".to_string()
        }
    ]
}