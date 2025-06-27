/**
 * EpicChain Rust Framework Website - Main JavaScript
 * Handles all interactive functionality
 */

(function() {
    'use strict';

    // =================================
    // Global Variables
    // =================================
    let isScrolling = false;
    let lastScrollY = 0;

    // =================================
    // Utility Functions
    // =================================
    
    /**
     * Debounce function to limit function calls
     */
    function debounce(func, wait = 20, immediate = true) {
        let timeout;
        return function executedFunction(...args) {
            const later = () => {
                timeout = null;
                if (!immediate) func(...args);
            };
            const callNow = immediate && !timeout;
            clearTimeout(timeout);
            timeout = setTimeout(later, wait);
            if (callNow) func(...args);
        };
    }

    /**
     * Throttle function to limit function calls
     */
    function throttle(func, limit) {
        let inThrottle;
        return function() {
            const args = arguments;
            const context = this;
            if (!inThrottle) {
                func.apply(context, args);
                inThrottle = true;
                setTimeout(() => inThrottle = false, limit);
            }
        };
    }

    /**
     * Check if element is in viewport
     */
    function isInViewport(element, threshold = 0.1) {
        const rect = element.getBoundingClientRect();
        const windowHeight = window.innerHeight || document.documentElement.clientHeight;
        const windowWidth = window.innerWidth || document.documentElement.clientWidth;
        
        return (
            rect.top >= -threshold * windowHeight &&
            rect.left >= -threshold * windowWidth &&
            rect.bottom <= windowHeight + threshold * windowHeight &&
            rect.right <= windowWidth + threshold * windowWidth
        );
    }

    /**
     * Smooth scroll to element
     */
    function smoothScrollTo(target, duration = 800) {
        const targetElement = typeof target === 'string' ? document.querySelector(target) : target;
        if (!targetElement) return;

        const targetPosition = targetElement.getBoundingClientRect().top + window.pageYOffset - 80;
        const startPosition = window.pageYOffset;
        const distance = targetPosition - startPosition;
        let startTime = null;

        function animation(currentTime) {
            if (startTime === null) startTime = currentTime;
            const timeElapsed = currentTime - startTime;
            const run = ease(timeElapsed, startPosition, distance, duration);
            window.scrollTo(0, run);
            if (timeElapsed < duration) requestAnimationFrame(animation);
        }

        // Easing function
        function ease(t, b, c, d) {
            t /= d / 2;
            if (t < 1) return c / 2 * t * t + b;
            t--;
            return -c / 2 * (t * (t - 2) - 1) + b;
        }

        requestAnimationFrame(animation);
    }

    // =================================
    // Navigation Functionality
    // =================================
    
    function initNavigation() {
        const navbar = document.getElementById('navbar');
        const navToggle = document.getElementById('navToggle');
        const navMenu = document.getElementById('navMenu');
        const navLinks = document.querySelectorAll('.nav-link[href^="#"]');

        // Handle scroll effects on navbar
        function handleNavbarScroll() {
            const scrollY = window.scrollY;
            
            if (scrollY > 50) {
                navbar.classList.add('scrolled');
            } else {
                navbar.classList.remove('scrolled');
            }

            // Hide/show navbar on scroll (mobile)
            if (window.innerWidth <= 768) {
                if (scrollY > lastScrollY && scrollY > 100) {
                    navbar.style.transform = 'translateY(-100%)';
                } else {
                    navbar.style.transform = 'translateY(0)';
                }
            }

            lastScrollY = scrollY;
        }

        // Mobile menu toggle
        function toggleMobileMenu() {
            const isOpen = navMenu.classList.contains('open');
            
            if (isOpen) {
                navMenu.classList.remove('open');
                navToggle.classList.remove('open');
                document.body.style.overflow = '';
            } else {
                navMenu.classList.add('open');
                navToggle.classList.add('open');
                document.body.style.overflow = 'hidden';
            }
        }

        // Smooth scroll for navigation links
        function handleNavClick(e) {
            e.preventDefault();
            const targetId = this.getAttribute('href');
            const targetElement = document.querySelector(targetId);
            
            if (targetElement) {
                smoothScrollTo(targetElement);
                
                // Close mobile menu if open
                if (navMenu.classList.contains('open')) {
                    toggleMobileMenu();
                }
            }
        }

        // Highlight active nav link
        function highlightActiveNavLink() {
            const sections = document.querySelectorAll('section[id]');
            const scrollPos = window.scrollY + 100;

            sections.forEach(section => {
                const sectionTop = section.offsetTop;
                const sectionHeight = section.offsetHeight;
                const sectionId = section.getAttribute('id');
                const navLink = document.querySelector(`.nav-link[href="#${sectionId}"]`);

                if (scrollPos >= sectionTop && scrollPos < sectionTop + sectionHeight) {
                    navLinks.forEach(link => link.classList.remove('active'));
                    if (navLink) navLink.classList.add('active');
                }
            });
        }

        // Event listeners
        window.addEventListener('scroll', throttle(handleNavbarScroll, 10));
        window.addEventListener('scroll', throttle(highlightActiveNavLink, 100));
        navToggle?.addEventListener('click', toggleMobileMenu);
        navLinks.forEach(link => link.addEventListener('click', handleNavClick));

        // Close mobile menu when clicking outside
        document.addEventListener('click', (e) => {
            if (navMenu.classList.contains('open') && 
                !navMenu.contains(e.target) && 
                !navToggle.contains(e.target)) {
                toggleMobileMenu();
            }
        });

        // Handle resize
        window.addEventListener('resize', debounce(() => {
            if (window.innerWidth > 768) {
                navMenu.classList.remove('open');
                navToggle.classList.remove('open');
                document.body.style.overflow = '';
                navbar.style.transform = 'translateY(0)';
            }
        }, 250));
    }

    // =================================
    // Examples Filtering
    // =================================
    
    function initExamplesFilter() {
        const filterButtons = document.querySelectorAll('.filter-btn');
        const exampleCards = document.querySelectorAll('.example-card');

        function filterExamples(category) {
            exampleCards.forEach(card => {
                const cardCategory = card.getAttribute('data-category');
                
                if (category === 'all' || cardCategory === category) {
                    card.style.display = 'block';
                    card.style.opacity = '0';
                    card.style.transform = 'translateY(20px)';
                    
                    // Animate in
                    setTimeout(() => {
                        card.style.transition = 'all 0.3s ease-out';
                        card.style.opacity = '1';
                        card.style.transform = 'translateY(0)';
                    }, 50);
                } else {
                    card.style.transition = 'all 0.2s ease-out';
                    card.style.opacity = '0';
                    card.style.transform = 'translateY(-20px)';
                    
                    setTimeout(() => {
                        card.style.display = 'none';
                    }, 200);
                }
            });
        }

        function handleFilterClick() {
            const category = this.getAttribute('data-filter');
            
            // Update active button
            filterButtons.forEach(btn => btn.classList.remove('active'));
            this.classList.add('active');
            
            // Filter examples
            filterExamples(category);
        }

        // Event listeners
        filterButtons.forEach(button => {
            button.addEventListener('click', handleFilterClick);
        });
    }

    // =================================
    // Scroll Animations
    // =================================
    
    function initScrollAnimations() {
        const animatedElements = document.querySelectorAll('.fade-in-up, .feature-card, .example-card, .doc-card, .arch-step, .component');

        function checkAnimations() {
            animatedElements.forEach(element => {
                if (isInViewport(element, 0.1) && !element.classList.contains('animated')) {
                    element.classList.add('animated');
                    element.style.opacity = '1';
                    element.style.transform = 'translateY(0)';
                }
            });
        }

        // Initialize elements
        animatedElements.forEach(element => {
            element.style.opacity = '0';
            element.style.transform = 'translateY(30px)';
            element.style.transition = 'all 0.6s ease-out';
        });

        // Check on scroll
        window.addEventListener('scroll', throttle(checkAnimations, 50));
        
        // Initial check
        checkAnimations();
    }

    // =================================
    // Hero Particles Animation
    // =================================
    
    function initHeroParticles() {
        const heroParticles = document.querySelector('.hero-particles');
        
        if (!heroParticles) return;

        function updateParticles() {
            const scrollY = window.pageYOffset;
            const speed = 0.5;
            heroParticles.style.transform = `translateY(${scrollY * speed}px)`;
        }

        window.addEventListener('scroll', throttle(updateParticles, 10));
    }

    // =================================
    // Code Block Enhancements & Prism Integration
    // =================================
    
    function initCodeBlocks() {
        // Wait for Prism to be fully loaded
        if (typeof Prism !== 'undefined') {
            // Custom Rust token enhancements for better highlighting
            if (Prism.languages.rust) {
                // Add epicchain-contract specific highlighting
                Prism.languages.rust = Prism.languages.extend('rust', {
                    'epicchain-attribute': {
                        pattern: /#\[(epicchain_contract|epicchain_contract_impl|safe)\]/,
                        alias: 'important',
                        greedy: true
                    },
                    'contract-keywords': {
                        pattern: /\b(rt::log|rt::check_witness|rt::get_invocation_counter)\b/,
                        alias: 'function'
                    },
                    'epicchain-types': {
                        pattern: /\b(H160|H256|ByteArray|Storage)\b/,
                        alias: 'class-name'
                    }
                });
            }
            
            // Force re-highlight all code blocks
            Prism.highlightAll();
        }
        
        const codeBlocks = document.querySelectorAll('pre[class*="language-"]');

        codeBlocks.forEach(block => {
            // Add copy button
            const copyButton = document.createElement('button');
            copyButton.className = 'copy-btn';
            copyButton.innerHTML = '📋';
            copyButton.title = 'Copy to clipboard';
            copyButton.setAttribute('aria-label', 'Copy code to clipboard');

            copyButton.addEventListener('click', async () => {
                // Get text content without syntax highlighting markup
                const code = block.querySelector('code')?.textContent || block.textContent;
                
                try {
                    await navigator.clipboard.writeText(code);
                    copyButton.innerHTML = '✅';
                    copyButton.title = 'Copied!';
                    copyButton.setAttribute('aria-label', 'Code copied to clipboard');
                    
                    // Announce to screen readers
                    const announcement = document.createElement('div');
                    announcement.setAttribute('aria-live', 'polite');
                    announcement.setAttribute('aria-atomic', 'true');
                    announcement.style.cssText = 'position: absolute; left: -10000px; width: 1px; height: 1px; overflow: hidden;';
                    announcement.textContent = 'Code copied to clipboard';
                    document.body.appendChild(announcement);
                    
                    setTimeout(() => {
                        copyButton.innerHTML = '📋';
                        copyButton.title = 'Copy to clipboard';
                        copyButton.setAttribute('aria-label', 'Copy code to clipboard');
                        document.body.removeChild(announcement);
                    }, 2000);
                } catch (err) {
                    console.error('Failed to copy code:', err);
                    copyButton.innerHTML = '❌';
                    copyButton.title = 'Failed to copy';
                    setTimeout(() => {
                        copyButton.innerHTML = '📋';
                        copyButton.title = 'Copy to clipboard';
                    }, 2000);
                }
            });

            // Add button to code block container
            const container = block.closest('.code-window') || block.parentElement;
            container.style.position = 'relative';
            container.appendChild(copyButton);

            // Enhanced copy button styling
            copyButton.style.cssText = `
                position: absolute;
                top: 0.75rem;
                right: 0.75rem;
                background: rgba(37, 38, 40, 0.95);
                border: 1px solid #3f4042;
                border-radius: 0.375rem;
                color: #a3a3a3;
                padding: 0.375rem 0.75rem;
                cursor: pointer;
                font-size: 0.75rem;
                font-weight: 500;
                transition: all 0.2s ease;
                opacity: 0;
                backdrop-filter: blur(10px);
                z-index: 10;
                user-select: none;
                outline: none;
            `;

            // Show/hide on hover with improved UX
            const showCopyButton = () => {
                copyButton.style.opacity = '1';
                copyButton.style.transform = 'translateY(0)';
            };

            const hideCopyButton = () => {
                if (!copyButton.matches(':focus')) {
                    copyButton.style.opacity = '0';
                    copyButton.style.transform = 'translateY(-4px)';
                }
            };

            container.addEventListener('mouseenter', showCopyButton);
            container.addEventListener('mouseleave', hideCopyButton);
            copyButton.addEventListener('focus', showCopyButton);
            copyButton.addEventListener('blur', hideCopyButton);

            // Enhanced hover effects
            copyButton.addEventListener('mouseenter', () => {
                copyButton.style.color = '#00d4ff';
                copyButton.style.borderColor = '#00d4ff';
                copyButton.style.background = 'rgba(0, 212, 255, 0.15)';
                copyButton.style.transform = 'translateY(0) scale(1.05)';
            });

            copyButton.addEventListener('mouseleave', () => {
                copyButton.style.color = '#a3a3a3';
                copyButton.style.borderColor = '#3f4042';
                copyButton.style.background = 'rgba(37, 38, 40, 0.95)';
                copyButton.style.transform = 'translateY(0) scale(1)';
            });

            // Keyboard support
            copyButton.addEventListener('keydown', (e) => {
                if (e.key === 'Enter' || e.key === ' ') {
                    e.preventDefault();
                    copyButton.click();
                }
            });
        });

        // Add syntax highlighting indicators
        const rustBlocks = document.querySelectorAll('pre.language-rust, pre[class*="language-rust"]');
        rustBlocks.forEach(block => {
            const indicator = document.createElement('span');
            indicator.textContent = 'Rust';
            indicator.className = 'language-indicator';
            indicator.style.cssText = `
                position: absolute;
                top: 0.75rem;
                left: 0.75rem;
                background: rgba(255, 107, 53, 0.15);
                color: #ff6b35;
                border: 1px solid rgba(255, 107, 53, 0.3);
                border-radius: 0.25rem;
                padding: 0.125rem 0.375rem;
                font-size: 0.625rem;
                font-weight: 600;
                text-transform: uppercase;
                letter-spacing: 0.05em;
                z-index: 5;
            `;
            
            const container = block.closest('.code-window') || block.parentElement;
            if (container && container.style.position !== 'static') {
                container.appendChild(indicator);
            }
        });
    }

    // =================================
    // Performance Optimizations
    // =================================
    
    function initPerformanceOptimizations() {
        // Lazy load images
        const images = document.querySelectorAll('img[data-src]');
        
        if ('IntersectionObserver' in window) {
            const imageObserver = new IntersectionObserver((entries, observer) => {
                entries.forEach(entry => {
                    if (entry.isIntersecting) {
                        const img = entry.target;
                        img.src = img.dataset.src;
                        img.classList.remove('lazy');
                        imageObserver.unobserve(img);
                    }
                });
            });

            images.forEach(img => imageObserver.observe(img));
        }

        // Prefetch critical resources
        const criticalLinks = [
            'https://fonts.googleapis.com',
            'https://cdnjs.cloudflare.com'
        ];

        criticalLinks.forEach(url => {
            const link = document.createElement('link');
            link.rel = 'dns-prefetch';
            link.href = url;
            document.head.appendChild(link);
        });
    }

    // =================================
    // Accessibility Enhancements
    // =================================
    
    function initAccessibility() {
        // Skip to main content link
        const skipLink = document.createElement('a');
        skipLink.href = '#hero';
        skipLink.textContent = 'Skip to main content';
        skipLink.className = 'skip-link';
        skipLink.style.cssText = `
            position: absolute;
            top: -40px;
            left: 6px;
            background: #00d4ff;
            color: #0a0a0b;
            padding: 8px;
            border-radius: 4px;
            text-decoration: none;
            z-index: 1100;
            font-weight: 600;
            transition: top 0.3s;
        `;
        
        skipLink.addEventListener('focus', () => {
            skipLink.style.top = '6px';
        });
        
        skipLink.addEventListener('blur', () => {
            skipLink.style.top = '-40px';
        });
        
        document.body.insertBefore(skipLink, document.body.firstChild);

        // Keyboard navigation for cards
        const cards = document.querySelectorAll('.feature-card, .example-card, .doc-card');
        
        cards.forEach(card => {
            card.setAttribute('tabindex', '0');
            card.setAttribute('role', 'button');
            
            card.addEventListener('keydown', (e) => {
                if (e.key === 'Enter' || e.key === ' ') {
                    e.preventDefault();
                    card.click();
                }
            });
        });

        // Announce page changes for screen readers
        const announcer = document.createElement('div');
        announcer.setAttribute('aria-live', 'polite');
        announcer.setAttribute('aria-atomic', 'true');
        announcer.style.cssText = `
            position: absolute;
            left: -10000px;
            width: 1px;
            height: 1px;
            overflow: hidden;
        `;
        document.body.appendChild(announcer);

        // Announce filter changes
        const filterButtons = document.querySelectorAll('.filter-btn');
        filterButtons.forEach(button => {
            button.addEventListener('click', () => {
                const category = button.getAttribute('data-filter');
                const categoryName = category === 'all' ? 'all examples' : category + ' examples';
                announcer.textContent = `Showing ${categoryName}`;
            });
        });
    }

    // =================================
    // Error Handling
    // =================================
    
    function initErrorHandling() {
        window.addEventListener('error', (e) => {
            console.error('Website error:', e.error);
            // Could send to analytics service here
        });

        window.addEventListener('unhandledrejection', (e) => {
            console.error('Unhandled promise rejection:', e.reason);
            // Could send to analytics service here
        });
    }

    // =================================
    // Feature Detection and Polyfills
    // =================================
    
    function initFeatureDetection() {
        // Add CSS class for JS enabled
        document.documentElement.classList.add('js-enabled');

        // Detect and handle reduced motion preference
        if (window.matchMedia('(prefers-reduced-motion: reduce)').matches) {
            document.documentElement.classList.add('reduce-motion');
        }

        // Detect touch devices
        if ('ontouchstart' in window || navigator.maxTouchPoints > 0) {
            document.documentElement.classList.add('touch-device');
        }

        // Detect high contrast mode
        if (window.matchMedia('(prefers-contrast: high)').matches) {
            document.documentElement.classList.add('high-contrast');
        }
    }

    // =================================
    // Analytics and Tracking
    // =================================
    
    function initAnalytics() {
        // Track button clicks
        const buttons = document.querySelectorAll('.btn, .filter-btn, .nav-link');
        
        buttons.forEach(button => {
            button.addEventListener('click', () => {
                const buttonText = button.textContent.trim();
                const buttonType = button.className;
                
                // Send to analytics (if available)
                if (typeof gtag !== 'undefined') {
                    gtag('event', 'click', {
                        event_category: 'Button',
                        event_label: buttonText,
                        value: buttonType
                    });
                }
            });
        });

        // Track scroll depth
        let scrollDepthTracked = [];
        const milestones = [25, 50, 75, 100];

        function trackScrollDepth() {
            const scrollPercent = Math.round(
                (window.scrollY / (document.body.scrollHeight - window.innerHeight)) * 100
            );

            milestones.forEach(milestone => {
                if (scrollPercent >= milestone && !scrollDepthTracked.includes(milestone)) {
                    scrollDepthTracked.push(milestone);
                    
                    if (typeof gtag !== 'undefined') {
                        gtag('event', 'scroll', {
                            event_category: 'Scroll Depth',
                            event_label: `${milestone}%`,
                            value: milestone
                        });
                    }
                }
            });
        }

        window.addEventListener('scroll', throttle(trackScrollDepth, 500));
    }

    // =================================
    // Initialization
    // =================================
    
    function init() {
        // Wait for DOM to be ready
        if (document.readyState === 'loading') {
            document.addEventListener('DOMContentLoaded', init);
            return;
        }

        try {
            // Initialize all modules
            initFeatureDetection();
            initNavigation();
            initExamplesFilter();
            initScrollAnimations();
            initHeroParticles();
            initCodeBlocks();
            initPerformanceOptimizations();
            initAccessibility();
            initErrorHandling();
            initAnalytics();

            // Initialize Feather icons
            if (typeof feather !== 'undefined') {
                feather.replace();
            }

            // Initialize Prism syntax highlighting
            if (typeof Prism !== 'undefined') {
                Prism.highlightAll();
            }

            console.log('EpicChain Rust Framework website initialized successfully');

        } catch (error) {
            console.error('Error initializing website:', error);
        }
    }

    // Start initialization
    init();

})(); 