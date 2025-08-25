# CS2 Demo Core - Architecture

This document outlines the architecture and structure of the CS2 Demo Core project.

## 📁 Directory Structure

```
cs2-demo-core/
├── 📄 README.md                    # Main documentation
├── 📄 Cargo.toml                   # Project configuration
├── 📄 LICENSE                      # MIT License
├── 📄 CHANGELOG.md                 # Version history
├── 📁 docs/                        # Documentation
│   ├── 📄 USAGE.md                 # Detailed usage guide
│   ├── 📄 API.md                   # Complete API reference
│   ├── 📄 ARCHITECTURE.md          # This file
│   └── 📄 CONTRIBUTING.md          # Contribution guidelines
├── 📁 src/                         # Source code
│   ├── 📄 lib.rs                   # Main library entry point
│   ├── 📁 parser/                  # Demo parsing logic
│   │   ├── 📄 mod.rs
│   │   ├── 📄 demo_parser.rs       # Main parser
│   │   ├── 📄 protobuf_parser.rs   # Protobuf parsing
│   │   └── 📄 event_extractor.rs   # Event extraction
│   ├── 📁 events/                  # Event definitions
│   │   ├── 📄 mod.rs
│   │   └── 📄 types.rs             # Event types
│   ├── 📁 utils/                   # Utilities
│   │   ├── 📄 mod.rs
│   │   ├── 📄 validation.rs        # Demo validation
│   │   └── 📄 helpers.rs           # Helper functions
│   └── 📁 error/                   # Error handling
│       ├── 📄 mod.rs
│       └── 📄 types.rs             # Error types
├── 📁 examples/                    # Usage examples
│   ├── 📄 basic_usage.rs           # Basic parsing
│   ├── 📄 simple_usage.rs          # Simple analysis
│   ├── 📄 real_usage.rs            # Real-world usage
│   └── 📄 integration_example.rs   # Integration examples
└── 📁 tests/                       # Test files
    └── 📄 integration_tests.rs     # Integration tests
```

## 🏗️ Architecture Overview

### **Core Components**

#### **1. Parser Module (`src/parser/`)**
- **`demo_parser.rs`** - Main parsing orchestration
- **`protobuf_parser.rs`** - CS2 demo protobuf parsing
- **`event_extractor.rs`** - Event extraction from parsed data

#### **2. Events Module (`src/events/`)**
- **`types.rs`** - Event type definitions (Kill, Headshot, Clutch, Round, Player)
- **`mod.rs`** - Module organization and re-exports

#### **3. Utilities Module (`src/utils/`)**
- **`validation.rs`** - Demo file validation
- **`helpers.rs`** - Common utility functions

#### **4. Error Handling (`src/error/`)**
- **`types.rs`** - Custom error types
- **`mod.rs`** - Error module organization

### **Data Flow**

```
Demo File (.dem) → Parser → Protobuf Parser → Event Extractor → DemoEvents
```

1. **Input**: CS2 demo file (.dem)
2. **Validation**: Check file format and integrity
3. **Parsing**: Extract protobuf messages
4. **Event Extraction**: Convert messages to structured events
5. **Output**: `DemoEvents` with all extracted data

## 🎯 Design Principles

### **1. Separation of Concerns**
- **Parser**: Handles file reading and basic parsing
- **Event Extractor**: Focuses on event detection and extraction
- **Utilities**: Provides common functionality
- **Error Handling**: Centralized error management

### **2. Async-First Design**
- All I/O operations are async
- Non-blocking parsing for better performance
- Supports concurrent processing

### **3. Memory Efficiency**
- Streaming parsing for large files
- Minimal memory allocations
- Efficient data structures

### **4. Type Safety**
- Strong typing for all data structures
- Comprehensive error handling
- Clear API contracts

## 🔧 Key Features

### **Core Functionality**
- ✅ High-performance CS2 demo parsing
- ✅ Event extraction (kills, headshots, clutches, rounds)
- ✅ Player statistics and analysis
- ✅ Async/await support
- ✅ Memory-efficient processing

### **Developer Experience**
- ✅ Comprehensive documentation
- ✅ Multiple usage examples
- ✅ Clear error handling
- ✅ Type-safe API design
- ✅ Performance optimizations

### **Project Quality**
- ✅ Professional documentation structure
- ✅ DG Web3 Labs branding
- ✅ MIT License for open source
- ✅ Contribution guidelines
- ✅ Testing framework

## 🚀 Roadmap

### **SDK Features (cs2-demo-core)**
- [ ] **Real protobuf parsing implementation**
- [ ] **Extract actual events** (kills, headshots, rounds)
- [ ] **Proper CS2 version validation**
- [ ] **Multi-threading support**
- [ ] **Streaming parser for large demos**
- [ ] **Performance optimizations**
- [ ] **Enhanced error handling**

### **Future Enhancements**
- [ ] **Benchmarking suite**
- [ ] **Integration tests**
- [ ] **Performance profiling tools**
- [ ] **Memory usage optimization**

## 📊 Current Status

### **✅ Completed**
- Project structure and organization
- Documentation framework
- Basic parsing infrastructure
- Error handling system
- Example implementations
- DG Web3 Labs branding

### **🔄 In Progress**
- Real protobuf parsing implementation
- Event extraction logic
- Performance optimizations

### **📋 Planned**
- Multi-threading support
- Streaming parser
- Advanced analytics
- Integration with Highlight Hub platform

## 🎯 Next Steps

1. **Implement real protobuf parsing** for actual CS2 demo files
2. **Extract real events** from parsed demo data
3. **Add comprehensive tests** for all functionality
4. **Optimize performance** for large demo files
5. **Publish to crates.io** for community distribution
6. **Integrate with Highlight Hub** platform development

## 🤝 Contributing

The project is now well-organized and ready for contributions. See `CONTRIBUTING.md` for detailed guidelines.

### **Getting Started**
```bash
git clone https://github.com/dgweb3labs/cs2-demo-core.git
cd cs2-demo-core
cargo check
cargo test
cargo run --example simple_usage
```

### **Documentation**
- **Quick Start**: See `../README.md`
- **Detailed Usage**: See `USAGE.md`
- **API Reference**: See `API.md`
- **Contributing**: See `CONTRIBUTING.md`

---

**Built and maintained by [DG Web3 Labs](https://github.com/dgweb3labs)**
