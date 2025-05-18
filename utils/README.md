# Utility Functions Documentation

## Particle Animation Utility (particleAnimation.js)

This utility function provides a reusable particle background animation effect that can be easily integrated into any React component.

### Usage

1. Import the particle animation utility function in your component:

```jsx
import { initParticleAnimation } from '../utils/particleAnimation';
```

2. Create a DOM reference to store the particle container:

```jsx
const particlesRef = useRef(null);
```

3. Initialize the particle animation in your component's useEffect:

```jsx
useEffect(() => {
  if (!particlesRef.current) return;
  
  // Initialize particle animation
  const cleanup = initParticleAnimation(particlesRef.current, {
    particleCount: 25,            // Number of particles
    color: 'rgba(0, 211, 217, 0.6)', // Particle color
    connect: true                 // Whether to connect particles
  });
  
  // Clean up particles when component unmounts
  return () => {
    if (cleanup) cleanup();
  };
}, []);
```

4. Add the particle container in your component's JSX:

```jsx
<div ref={particlesRef} className="moving-particles"></div>
```

5. Make sure your CSS includes the necessary styles:

```css
.moving-particles {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  overflow: hidden;
  z-index: 0;
}
```

### Configuration Options

The particle animation can be customized with the following options:

| Option | Type | Default | Description |
|------|------|--------|------|
| particleCount | Number | 30 | Number of particles |
| color | String | 'rgba(0, 211, 217, 0.5)' | Color of particles (RGBA format) |
| minSize | Number | 2 | Minimum particle size (in pixels) |
| maxSize | Number | 5 | Maximum particle size (in pixels) |
| minSpeed | Number | 0.1 | Minimum particle speed |
| maxSpeed | Number | 0.4 | Maximum particle speed |
| connect | Boolean | false | Whether to draw connection lines between particles |
| connectionDistance | Number | 150 | Maximum distance for particle connections |
| connectionOpacity | Number | 0.1 | Opacity of connection lines |

### Example

Example usage in a HeroSection component:

```jsx
import React, { useRef, useEffect } from 'react';
import { initParticleAnimation } from '../utils/particleAnimation';
import './HeroSection.css';

const HeroSection = () => {
  const particlesRef = useRef(null);
  
  useEffect(() => {
    if (!particlesRef.current) return;
    
    const cleanup = initParticleAnimation(particlesRef.current, {
      particleCount: 25,
      color: 'rgba(0, 211, 217, 0.6)',
      connect: true,
      minSize: 2,
      maxSize: 5
    });
    
    return () => {
      if (cleanup) cleanup();
    };
  }, []);
  
  return (
    <section className="hero-section">
      <div className="background-grid"></div>
      <div ref={particlesRef} className="moving-particles"></div>
      
      {/* Other content */}
    </section>
  );
};

export default HeroSection;
``` 