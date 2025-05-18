/**
 * Scroll Animation Utility Functions
 * For detecting when elements enter the viewport and triggering animations
 */

/**
 * Determine if an element has entered the viewport
 * @param {HTMLElement} element - DOM element to check
 * @param {number} threshold - Viewport entry threshold ratio, default 0.2 (20%)
 * @returns {boolean} - Whether the element is in the viewport
 */
export const isElementInViewport = (element, threshold = 0.2) => {
  if (!element) return false;
  
  const rect = element.getBoundingClientRect();
  const windowHeight = window.innerHeight || document.documentElement.clientHeight;
  
  // Modified detection logic to lower the threshold for element visibility
  // Start animation as soon as element top enters viewport
  const vertInView = rect.top <= windowHeight * 0.9;
  
  console.log(`Element visibility check: ${vertInView ? 'visible' : 'not visible'} (top: ${rect.top}, windowHeight: ${windowHeight})`);
  
  return vertInView;
};

/**
 * Initialize scroll monitoring to add animation classes to elements entering viewport
 * @param {string} targetSelector - CSS selector for target elements
 * @param {string} animatedClass - CSS class to add, default is 'animated'
 * @param {number} threshold - Viewport entry threshold ratio, default 0.2
 * @returns {Function} - Cleanup function to remove event listeners
 */
export const initScrollAnimation = (targetSelector, animatedClass = 'animated', threshold = 0.2) => {
  const animateOnScroll = () => {
    const elements = document.querySelectorAll(targetSelector);
    console.log(`Checking ${elements.length} elements with selector "${targetSelector}"`);
    
    elements.forEach((element, index) => {
      const isVisible = isElementInViewport(element, threshold);
      const hasClass = element.classList.contains(animatedClass);
      
      console.log(`Element ${index}: visible=${isVisible}, hasClass=${hasClass}`);
      
      if (isVisible && !hasClass) {
        console.log(`Adding class "${animatedClass}" to element ${index}`);
        element.classList.add(animatedClass);
      }
    });
  };
  
  // Initial check of currently visible elements
  console.log('Initial animation check');
  animateOnScroll();
  
  // Add scroll event listener
  console.log('Adding scroll event listener');
  window.addEventListener('scroll', animateOnScroll, { passive: true });
  
  // Add additional resize listener to check elements when window size changes
  window.addEventListener('resize', animateOnScroll, { passive: true });
  
  // Delayed second check to ensure DOM is fully loaded and rendered
  setTimeout(animateOnScroll, 500);
  
  // Return a cleanup function
  return () => {
    window.removeEventListener('scroll', animateOnScroll);
    window.removeEventListener('resize', animateOnScroll);
    console.log('Removed event listeners');
  };
};

/**
 * Add scroll monitoring to container elements, adding specified class when they enter viewport
 * @param {string} containerSelector - CSS selector for container elements
 * @param {string} viewClass - CSS class to add, default is 'in-view'
 * @param {number} threshold - Viewport entry threshold ratio, default 0.2
 * @returns {Function} - Cleanup function to remove event listeners
 */
export const initSectionAnimation = (containerSelector, viewClass = 'in-view', threshold = 0.2) => {
  const animateSectionOnScroll = () => {
    const containers = document.querySelectorAll(containerSelector);
    console.log(`Checking ${containers.length} containers with selector "${containerSelector}"`);
    
    containers.forEach((container, index) => {
      const isVisible = isElementInViewport(container, threshold);
      const hasClass = container.classList.contains(viewClass);
      
      console.log(`Container ${index}: visible=${isVisible}, hasClass=${hasClass}`);
      
      if (isVisible && !hasClass) {
        console.log(`Adding class "${viewClass}" to container ${index}`);
        container.classList.add(viewClass);
      }
    });
  };
  
  // Initial check of currently visible elements
  console.log('Initial section check');
  animateSectionOnScroll();
  
  // Add scroll event listener
  console.log('Adding scroll event listener for sections');
  window.addEventListener('scroll', animateSectionOnScroll, { passive: true });
  
  // Add additional resize listener to check elements when window size changes
  window.addEventListener('resize', animateSectionOnScroll, { passive: true });
  
  // Delayed second check to ensure DOM is fully loaded and rendered
  setTimeout(animateSectionOnScroll, 500);
  
  // Return a cleanup function
  return () => {
    window.removeEventListener('scroll', animateSectionOnScroll);
    window.removeEventListener('resize', animateSectionOnScroll);
    console.log('Removed section event listeners');
  };
}; 