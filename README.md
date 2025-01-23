# Avalanche Detection and Risk Analyzer

A modern desktop application built with Rust that uses AI to detect and analyze avalanche risks in mountain terrain images.

## Features

- **Real-time Image Analysis**: Upload and analyze mountain terrain images instantly
- **AI-Powered Classification**: Utilizes OpenAI's GPT-4 Vision for accurate avalanche risk assessment
- **Comprehensive Analysis**:
  - Snow texture analysis (granular, blocky, fluffy)
  - Terrain feature detection
  - Movement pattern prediction
  - Slope angle assessment
- **Modern UI**: Clean, iOS-inspired interface with intuitive controls and visual feedback
- **Risk Confidence**: Visual confidence indicators with color-coded risk levels

## Prerequisites

- Rust (latest stable version)
- OpenAI API Key
- Cargo package manager

## Installation

1. Clone the repository:
```bash
git clone https://github.com/ronnakamoto/avalanche-classifier.git
cd avalanche-classifier
```

2. Install dependencies:
```bash
cargo build
```

3. Run the application:
```bash
cargo run
```

## Usage

1. Launch the application
2. Enter your OpenAI API key in the provided field
3. Click "Upload Mountain Image" to select an image for analysis
4. Click "Analyze Terrain Risk" to start the analysis
5. Review the detailed results:
   - Overall avalanche risk assessment
   - Confidence level
   - Snow characteristics
   - Terrain features
   - Movement patterns

## Technical Details

### Dependencies

- `eframe`: Egui framework for cross-platform GUI
- `tokio`: Async runtime for network operations
- `reqwest`: HTTP client for API communication
- `serde`: Serialization/deserialization of JSON
- `image`: Image processing and manipulation
- `rfd`: Native file dialogs

### Architecture

The application follows a clean, modular architecture:

- **UI Layer**: Built with egui for responsive and native-feeling interface
- **Analysis Engine**: Async processing of images and API communication
- **Data Models**: Strongly-typed structures for avalanche risk data
- **Error Handling**: Comprehensive error management and user feedback

## Safety Notice ⚠️

This tool is designed to assist in avalanche risk assessment but should not be used as the sole decision-making tool for backcountry activities. Always:

- Consult professional weather services
- Check local avalanche forecasts
- Travel with proper safety equipment
- Make decisions based on multiple sources of information

## License 

MIT License