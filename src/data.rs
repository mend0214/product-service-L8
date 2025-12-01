use crate::model::Product;
use crate::configuration::Settings;

pub fn fetch_products(_settings: &Settings) -> Vec<Product> {
    vec![
        Product {
            id: 1,
            name: "SteelSeries Apex Pro TKL Mechanical Keyboard".to_string(),
            price: 249.99,
            description: "Make a significant increase in your gaming speed and accuracy with this Steelseries Apex Pro TKL keyboard. Equipped with OmniPoint 2.0 switches, this mechanical keyboard can deliver about 10 times faster actuation and 11 times quicker response. Adjustable keystrokes let you precisely adjust registration depth from anywhere between 0.2mm to 3.8mm.".to_string(),
            image: "/apex-pro-tkl.jpg".to_string()
        },
        Product {
            id: 2,
            name: "Razer Huntsman V3 Pro Mini Optical Gaming Keyboard".to_string(),
            price: 219.99,
            description: "Always have an edge in the competition with this Razer Huntsman V3 Pro Mini backlit analog optical gaming keyboard. Its 60% mini design frees up more of your desk space, and the brushed aluminum top ensures great durability. Analog optical switches Gen-2 with Rapid Trigger and adjustable actuation help let you enjoy top-notch speed during gaming.".to_string(),
            image: "/razer-huntsman-v3pro.jpg".to_string()
        },
        Product {
            id: 3,
            name: "Logitech G PRO X TKL LIGHTSPEED Mechanical Gaming Keyboard".to_string(),
            price: 279.99,
            description: "Unlock the path to victory with the Logitech G PRO X TKL gaming keyboard. Designed by collaborating with the world’s best esports players, it comes with dual-shot PBT keycaps that are resistant to wear and tactile mechanical switches for greater precision and quiet operation. This gaming keyboard is equipped with LIGHTSPEED wireless technology for fast and lag-free connectivity.".to_string(),
            image: "/logitech-gpro-keyboard.jpg".to_string()
        },
        Product {
            id: 4,
            name: "Logitech G PRO X Superlight 2 Wireless Gaming Mouse".to_string(),
            price: 239.99,
            description: "Get superior precision and ergonomics for your esport adventures with the Logitech G PRO X Superlight 2 wireless gaming mouse. It tracks your smallest move on almost any surface with its 44,000 DPI HERO 2 sensor while the LIGHTFORCE hybrid optical-mechanical switches actuate at ultralow latencies. Along with USB-C charging, this mouse gives you up to 95 hours of battery durability.".to_string(),
            image: "/logitech-superlight2".to_string()
        },
        Product {
            id: 5,
            name: "Razer Viper V3 Pro Wireless Gaming Mouse - Black".to_string(),
            price: 179.99,
            description: "Improve your gaming skills with this Razer Viper V3 Pro wireless gaming mouse. Featuring Razer Focus Pro Optical Sensor Gen-2, this mouse guarantees impressive levels of precision. The Razer 8,000Hz HyperPolling Technology enables extremely low-latency response, and the Razer HyperSpeed wireless technology assures highly reliable and seamless connection.".to_string(),
            image: "/razer-v3-pro.jpg".to_string()
        },
        Product {
            id: 6,
            name: "Logitech G305 12000 DPI Wireless Optical Gaming Mouse - White".to_string(),
            price: 39.99,
            description: "Experience next-level gaming with the Logitech G305 wireless optical mouse. It features a HERO sensor with up to 12,000dpi for ultra-precise tracking and LIGHTSPEED wireless technology that delivers a 1ms response time. A lightweight 99g design combined with six programmable buttons ensures agile control and personalized gameplay for any style.".to_string(),
            image: "/logitech-g305.jpg".to_string()
        },
        Product {
            id: 7,
            name: "Hyperx Cloud III S Wireless Gaming Headset with Microphone for Multi-Platform - Black".to_string(),
            price: 169.99,
            description: "Experience next-level gaming audio with the Hyperx Cloud III S wireless gaming headset. Enjoy immersive sound from 53mm angled drivers and DTS Headphone:X spatial audio. Stay connected through 2.4GHz and Bluetooth, with up to 200 hours of battery life for uninterrupted use. The 10mm microphone ensures crisp call quality and clear communication with teammates.".to_string(),
            image: "/hyperx-cloud-3.jpg".to_string()
        },
        Product {
            id: 8,
            name: "Logitech G Pro X Gaming Headset with Microphone - Black".to_string(),
            price: 119.99,
            description: "Get the edge over the competition with this Logitech Pro X gaming headset. Designed for serious gamers, it boasts 50mm drivers, Blue VO!CE software, and next-generation 7.1 surround sound for tournament-level audio. It features a comfortable design with memory foam ear pads and two sets of covers for comfort during even the longest gaming sessions.".to_string(),
            image: "/logitech-gpro-headset.jpg".to_string()
        },
        Product {
            id: 9,
            name: "Acer Nitro N60-640-EB24 Desktop Gaming PC (Intel Core i7-14700F/32GB RAM/2TB SSD/NVIDIA GeForce RTX 5070/Windows 11)".to_string(),
            price: 1899.99,
            description: "Power up your gameplay like never before with the Acer Nitro N60-640-EB24 gaming PC. This desktop delivers top-tier performance with its Intel Core i7-14700F processor and NVIDIA GeForce RTX 5070 graphics. Enjoy smooth, lag-free sessions with 32GB DDR5 memory and a lightning-fast 2TB solid state drive. Stay connected with Wi-Fi 6 and Bluetooth 5.3 for seamless online play.".to_string(),
            image: "/acer-pc.jpg".to_string()
        },
        Product {
            id: 10,
            name: "SK Gaming RGB Desktop Tower PC- Intel Core i7 Skylake- 6700 3.4GHz / 16 GB RAM / 1TB SSD /Nvidia GeForce RTX3050/ Windows 11 Pro/White".to_string(),
            price: 899.99,
            description: "SK Gaming RGB Desktop Tower PC- Intel Core i7 Skylake- 6700 3.4GHz / 16 GB RAM / 1TB SSD /Nvidia GeForce RTX3050 / Windows 11 Pro".to_string(),
            image: "/sk-pc.png".to_string()
        }
    ]
}