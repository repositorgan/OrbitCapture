# OrbitCapture
Capture a photo with your curser.

> “Draw a loose shape with your mouse or trackpad—capture everything inside it instantly.”

## Product

**Title**

> Orbit Capture
Gesture-Based Screen Capture for Professionals

**Problem**

> Screen capture tools interrupt flow:

> Mode switching

> Rigid rectangles

> Toolbar hunting

> Professionals lose time every day.

Slide 3 — Solution

Draw a loose shape.
Get an instant capture.

No modes.
No menus.
No precision required.

Slide 4 — Why It’s Different

Gesture-driven

Freeform capture

Native performance

Works with existing tools

Slide 5 — Who It’s For

Designers

Developers

Analysts

Product managers

Documentation teams

Slide 6 — Technology

Real-time gesture recognition

Native screen capture

Cross-operating system architecture

Privacy-first (on-device)

Slide 7 — Business Model

Business-to-Consumer (B2C):

One-time license

Business-to-Business (B2B):

Per-seat annual licenses

Enterprise controls

Slide 8 — Why Now

Remote work

Visual communication

Design-heavy workflows

Users want speed, not features

Slide 9 — Vision

Make screen capture muscle memory.

## Outline 

orbit-capture/
├─ core/
│  ├─ gesture/
│  │  ├─ sampler.rs
│  │  ├─ detector.rs
│  │  ├─ hull.rs
│  │  └─ classifier.rs
│  ├─ geometry/
│  │  ├─ mask.rs
│  │  ├─ ellipse.rs
│  │  └─ polygon.rs
│  ├─ capture/
│  │  ├─ region.rs
│  │  └─ compositor.rs
│  ├─ export/
│  │  ├─ clipboard.rs
│  │  ├─ png.rs
│  │  └─ svg.rs
│  └─ lib.rs
│
├─ platform/
│  ├─ windows/
│  ├─ macos/
│  └─ linux/
│
├─ ui/
│  └─ tauri/
│
└─ api/
   └─ c_ffi.rs

## Roadmap

> “I built a gesture-recognition system that infers user intent from noisy real-world input and translates it into precise, low-latency system actions across multiple operating systems.”

