// API Client for Watchdog
const API_BASE_URL = ''; // Use same origin (proxy through Express server)

// DOM Elements
const endpointItems = document.querySelectorAll('.endpoint-item');
const endpointDetails = document.querySelectorAll('.endpoint-details');

// Custom Modal Elements
const modal = document.getElementById('custom-modal');
const modalMessage = document.getElementById('modal-message');
const closeModalBtn = document.querySelector('.close-modal');
const modalOkBtn = document.getElementById('modal-ok-btn');

// Show custom modal
function showModal(message) {
    modalMessage.textContent = message;
    modal.style.display = 'block';
    modal.style.animation = 'fadeIn 0.3s ease';
}

// Close modal
function closeModal() {
    modal.style.animation = 'fadeOut 0.3s ease';
    setTimeout(() => {
        modal.style.display = 'none';
    }, 300);
}

// Add animation to response box
function animateResponseBox(element) {
    element.classList.remove('updated');
    void element.offsetWidth; // Trigger reflow
    element.classList.add('updated');
}

// Event Listeners for modal
closeModalBtn.addEventListener('click', closeModal);
modalOkBtn.addEventListener('click', closeModal);

// Close modal when clicking outside
window.addEventListener('click', (event) => {
    if (event.target === modal) {
        closeModal();
    }
});

// Add fadeOut animation to CSS dynamically
const style = document.createElement('style');
style.innerHTML = `
    @keyframes fadeOut {
        from { opacity: 1; }
        to { opacity: 0; }
    }
`;
document.head.appendChild(style);

// Event Listeners for endpoint navigation
endpointItems.forEach(item => {
    item.addEventListener('click', () => {
        const endpoint = item.getAttribute('data-endpoint');
        
        // Update active state for sidebar items
        endpointItems.forEach(i => i.classList.remove('active'));
        item.classList.add('active');
        
        // Show the selected endpoint details with animation
        endpointDetails.forEach(detail => {
            detail.classList.remove('active');
            if (detail.id === `${endpoint}-details`) {
                // Add a slight delay for smoother transition
                setTimeout(() => {
                    detail.classList.add('active');
                }, 50);
            }
        });
    });
});

// Health Check
document.getElementById('health-check-btn').addEventListener('click', async () => {
    const button = document.getElementById('health-check-btn');
    const responseBox = document.getElementById('health-response');
    button.classList.add('btn-loading');
    
    try {
        const response = await fetch(`${API_BASE_URL}/health`, {
            method: 'GET'
        });
        
        // Check if the response is OK
        if (!response.ok) {
            throw new Error(`HTTP error! status: ${response.status}`);
        }
        
        // Try to parse as JSON
        let data;
        const contentType = response.headers.get('content-type');
        if (contentType && contentType.includes('application/json')) {
            data = await response.json();
        } else {
            // If not JSON, get as text
            const text = await response.text();
            throw new Error(`Expected JSON but received: ${text.substring(0, 100)}...`);
        }
        
        responseBox.textContent = JSON.stringify(data, null, 2);
        animateResponseBox(responseBox);
    } catch (error) {
        responseBox.textContent = `Error: ${error.message}`;
        animateResponseBox(responseBox);
    } finally {
        button.classList.remove('btn-loading');
    }
});

// Subscriptions
document.getElementById('get-subscriptions-btn').addEventListener('click', async () => {
    const userId = document.getElementById('get-user-id').value;
    const button = document.getElementById('get-subscriptions-btn');
    const responseBox = document.getElementById('subscriptions-response');
    button.classList.add('btn-loading');
    
    if (!userId) {
        showModal('Please enter a user ID');
        button.classList.remove('btn-loading');
        return;
    }
    
    try {
        const response = await fetch(`${API_BASE_URL}/api/v1/subscriptions/${userId}`, {
            method: 'GET'
        });
        
        // Handle case where user has no subscriptions (404)
        if (response.status === 404) {
            responseBox.textContent = "No subscriptions found for this user.";
            animateResponseBox(responseBox);
            return;
        }
        
        // Check if the response is OK for other cases
        if (!response.ok) {
            throw new Error(`HTTP error! status: ${response.status}`);
        }
        
        // Handle empty response
        const text = await response.text();
        if (!text) {
            responseBox.textContent = "No subscriptions found for this user.";
            animateResponseBox(responseBox);
            return;
        }
        
        // Try to parse as JSON
        const contentType = response.headers.get('content-type');
        if (contentType && contentType.includes('application/json')) {
            const data = JSON.parse(text);
            responseBox.textContent = JSON.stringify(data, null, 2);
        } else {
            responseBox.textContent = `Expected JSON but received: ${text.substring(0, 100)}...`;
        }
        animateResponseBox(responseBox);
    } catch (error) {
        responseBox.textContent = `Error: ${error.message}`;
        animateResponseBox(responseBox);
    } finally {
        button.classList.remove('btn-loading');
    }
});

document.getElementById('create-subscription-btn').addEventListener('click', async () => {
    const userId = document.getElementById('create-user-id').value;
    const subscriptionId = document.getElementById('subscription-id').value;
    const keywordsInput = document.getElementById('keywords').value;
    const button = document.getElementById('create-subscription-btn');
    const responseBox = document.getElementById('subscriptions-response');
    button.classList.add('btn-loading');
    
    if (!userId || !subscriptionId) {
        showModal('Please enter both User ID and Subscription ID');
        button.classList.remove('btn-loading');
        return;
    }
    
    const keywords = keywordsInput ? keywordsInput.split(',').map(k => k.trim()) : [];
    
    const requestBody = {
        user_id: userId,
        subscription_id: subscriptionId,
        keywords: keywords
    };
    
    try {
        const response = await fetch(`${API_BASE_URL}/api/v1/subscriptions`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify(requestBody)
        });
        
        // Check if the response is OK
        if (!response.ok) {
            throw new Error(`HTTP error! status: ${response.status}`);
        }
        
        // Try to parse as JSON
        const contentType = response.headers.get('content-type');
        if (contentType && contentType.includes('application/json')) {
            const data = await response.json();
            responseBox.textContent = JSON.stringify(data, null, 2);
        } else {
            const text = await response.text();
            responseBox.textContent = text || "Subscription created successfully.";
        }
        animateResponseBox(responseBox);
    } catch (error) {
        responseBox.textContent = `Error: ${error.message}`;
        animateResponseBox(responseBox);
    } finally {
        button.classList.remove('btn-loading');
    }
});

document.getElementById('delete-subscription-btn').addEventListener('click', async () => {
    const userId = document.getElementById('delete-user-id').value;
    const subscriptionId = document.getElementById('delete-subscription-id').value;
    const button = document.getElementById('delete-subscription-btn');
    const responseBox = document.getElementById('subscriptions-response');
    button.classList.add('btn-loading');
    
    if (!userId || !subscriptionId) {
        showModal('Please enter both User ID and Subscription ID');
        button.classList.remove('btn-loading');
        return;
    }
    
    try {
        const response = await fetch(`${API_BASE_URL}/api/v1/subscriptions/${userId}/${subscriptionId}`, {
            method: 'DELETE'
        });
        
        // Check if the response is OK
        if (!response.ok) {
            throw new Error(`HTTP error! status: ${response.status}`);
        }
        
        // Try to parse as JSON
        const contentType = response.headers.get('content-type');
        if (contentType && contentType.includes('application/json')) {
            const data = await response.json();
            responseBox.textContent = JSON.stringify(data, null, 2);
        } else {
            const text = await response.text();
            responseBox.textContent = text || "Subscription deleted successfully.";
        }
        animateResponseBox(responseBox);
    } catch (error) {
        responseBox.textContent = `Error: ${error.message}`;
        animateResponseBox(responseBox);
    } finally {
        button.classList.remove('btn-loading');
    }
});

// Fetchers
document.getElementById('get-fetcher-types-btn').addEventListener('click', async () => {
    const button = document.getElementById('get-fetcher-types-btn');
    const responseBox = document.getElementById('fetchers-response');
    button.classList.add('btn-loading');
    
    try {
        const response = await fetch(`${API_BASE_URL}/api/v1/fetchers/types`, {
            method: 'GET'
        });
        
        // Check if the response is OK
        if (!response.ok) {
            throw new Error(`HTTP error! status: ${response.status}`);
        }
        
        // Try to parse as JSON
        const contentType = response.headers.get('content-type');
        if (contentType && contentType.includes('application/json')) {
            const data = await response.json();
            responseBox.textContent = JSON.stringify(data, null, 2);
        } else {
            const text = await response.text();
            responseBox.textContent = text || "No fetcher types found.";
        }
        animateResponseBox(responseBox);
    } catch (error) {
        responseBox.textContent = `Error: ${error.message}`;
        animateResponseBox(responseBox);
    } finally {
        button.classList.remove('btn-loading');
    }
});

document.getElementById('get-fetchers-btn').addEventListener('click', async () => {
    const userId = document.getElementById('get-fetchers-user-id').value;
    const button = document.getElementById('get-fetchers-btn');
    const responseBox = document.getElementById('fetchers-response');
    button.classList.add('btn-loading');
    
    if (!userId) {
        showModal('Please enter a user ID');
        button.classList.remove('btn-loading');
        return;
    }
    
    try {
        const response = await fetch(`${API_BASE_URL}/api/v1/fetchers/${userId}`, {
            method: 'GET'
        });
        
        // Handle case where user has no fetchers (404)
        if (response.status === 404) {
            responseBox.textContent = "No fetchers found for this user.";
            animateResponseBox(responseBox);
            return;
        }
        
        // Check if the response is OK for other cases
        if (!response.ok) {
            throw new Error(`HTTP error! status: ${response.status}`);
        }
        
        // Handle empty response
        const text = await response.text();
        if (!text) {
            responseBox.textContent = "No fetchers found for this user.";
            animateResponseBox(responseBox);
            return;
        }
        
        // Try to parse as JSON
        const contentType = response.headers.get('content-type');
        if (contentType && contentType.includes('application/json')) {
            const data = JSON.parse(text);
            responseBox.textContent = JSON.stringify(data, null, 2);
        } else {
            responseBox.textContent = `Expected JSON but received: ${text.substring(0, 100)}...`;
        }
        animateResponseBox(responseBox);
    } catch (error) {
        responseBox.textContent = `Error: ${error.message}`;
        animateResponseBox(responseBox);
    } finally {
        button.classList.remove('btn-loading');
    }
});

document.getElementById('add-fetcher-btn').addEventListener('click', async () => {
    const userId = document.getElementById('add-fetcher-user-id').value;
    const fetcherName = document.getElementById('fetcher-name').value;
    const fetcherType = document.getElementById('fetcher-type').value;
    const subscriptionId = document.getElementById('fetcher-subscription-id').value;
    const button = document.getElementById('add-fetcher-btn');
    const responseBox = document.getElementById('fetchers-response');
    button.classList.add('btn-loading');
    
    if (!userId || !fetcherName || !fetcherType || !subscriptionId) {
        showModal('Please fill in all fields');
        button.classList.remove('btn-loading');
        return;
    }
    
    const requestBody = {
        user_id: userId,
        fetcher_name: fetcherName,
        fetcher_type: fetcherType,
        subscription_id: subscriptionId
    };
    
    try {
        const response = await fetch(`${API_BASE_URL}/api/v1/fetchers`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify(requestBody)
        });
        
        // Check if the response is OK
        if (!response.ok) {
            throw new Error(`HTTP error! status: ${response.status}`);
        }
        
        // Try to parse as JSON
        const contentType = response.headers.get('content-type');
        if (contentType && contentType.includes('application/json')) {
            const data = await response.json();
            responseBox.textContent = JSON.stringify(data, null, 2);
        } else {
            const text = await response.text();
            responseBox.textContent = text || "Fetcher added successfully.";
        }
        animateResponseBox(responseBox);
    } catch (error) {
        responseBox.textContent = `Error: ${error.message}`;
        animateResponseBox(responseBox);
    } finally {
        button.classList.remove('btn-loading');
    }
});

document.getElementById('delete-fetcher-btn').addEventListener('click', async () => {
    const userId = document.getElementById('delete-fetcher-user-id').value;
    const fetcherName = document.getElementById('fetcher-name-delete').value;
    const button = document.getElementById('delete-fetcher-btn');
    const responseBox = document.getElementById('fetchers-response');
    button.classList.add('btn-loading');
    
    if (!userId || !fetcherName) {
        showModal('Please enter both User ID and Fetcher Name');
        button.classList.remove('btn-loading');
        return;
    }
    
    try {
        const response = await fetch(`${API_BASE_URL}/api/v1/fetchers/${userId}/${fetcherName}`, {
            method: 'DELETE'
        });
        
        // Check if the response is OK
        if (!response.ok) {
            throw new Error(`HTTP error! status: ${response.status}`);
        }
        
        // Try to parse as JSON
        const contentType = response.headers.get('content-type');
        if (contentType && contentType.includes('application/json')) {
            const data = await response.json();
            responseBox.textContent = JSON.stringify(data, null, 2);
        } else {
            const text = await response.text();
            responseBox.textContent = text || "Fetcher deleted successfully.";
        }
        animateResponseBox(responseBox);
    } catch (error) {
        responseBox.textContent = `Error: ${error.message}`;
        animateResponseBox(responseBox);
    } finally {
        button.classList.remove('btn-loading');
    }
});

// Notifiers
// Toggle email field based on notifier type
document.getElementById('notifier-type').addEventListener('change', function() {
    const emailField = document.getElementById('email-field');
    if (this.value === 'ArxivEmailNotifier') {
        emailField.style.display = 'block';
        emailField.style.animation = 'fadeIn 0.3s ease';
    } else {
        emailField.style.animation = 'fadeOut 0.3s ease';
        setTimeout(() => {
            emailField.style.display = 'none';
        }, 300);
    }
});

document.getElementById('get-notifier-types-btn').addEventListener('click', async () => {
    const button = document.getElementById('get-notifier-types-btn');
    const responseBox = document.getElementById('notifiers-response');
    button.classList.add('btn-loading');
    
    try {
        const response = await fetch(`${API_BASE_URL}/api/v1/notifiers/types`, {
            method: 'GET'
        });
        
        // Check if the response is OK
        if (!response.ok) {
            throw new Error(`HTTP error! status: ${response.status}`);
        }
        
        // Try to parse as JSON
        const contentType = response.headers.get('content-type');
        if (contentType && contentType.includes('application/json')) {
            const data = await response.json();
            responseBox.textContent = JSON.stringify(data, null, 2);
        } else {
            const text = await response.text();
            responseBox.textContent = text || "No notifier types found.";
        }
        animateResponseBox(responseBox);
    } catch (error) {
        responseBox.textContent = `Error: ${error.message}`;
        animateResponseBox(responseBox);
    } finally {
        button.classList.remove('btn-loading');
    }
});

document.getElementById('get-notifiers-btn').addEventListener('click', async () => {
    const userId = document.getElementById('get-notifiers-user-id').value;
    const button = document.getElementById('get-notifiers-btn');
    const responseBox = document.getElementById('notifiers-response');
    button.classList.add('btn-loading');
    
    if (!userId) {
        showModal('Please enter a user ID');
        button.classList.remove('btn-loading');
        return;
    }
    
    try {
        const response = await fetch(`${API_BASE_URL}/api/v1/notifiers/${userId}`, {
            method: 'GET'
        });
        
        // Handle case where user has no notifiers (404)
        if (response.status === 404) {
            responseBox.textContent = "No notifiers found for this user.";
            animateResponseBox(responseBox);
            return;
        }
        
        // Check if the response is OK for other cases
        if (!response.ok) {
            throw new Error(`HTTP error! status: ${response.status}`);
        }
        
        // Handle empty response
        const text = await response.text();
        if (!text) {
            responseBox.textContent = "No notifiers found for this user.";
            animateResponseBox(responseBox);
            return;
        }
        
        // Try to parse as JSON
        const contentType = response.headers.get('content-type');
        if (contentType && contentType.includes('application/json')) {
            const data = JSON.parse(text);
            responseBox.textContent = JSON.stringify(data, null, 2);
        } else {
            responseBox.textContent = `Expected JSON but received: ${text.substring(0, 100)}...`;
        }
        animateResponseBox(responseBox);
    } catch (error) {
        responseBox.textContent = `Error: ${error.message}`;
        animateResponseBox(responseBox);
    } finally {
        button.classList.remove('btn-loading');
    }
});

document.getElementById('add-notifier-btn').addEventListener('click', async () => {
    const userId = document.getElementById('add-notifier-user-id').value;
    const notifierName = document.getElementById('notifier-name').value;
    const notifierType = document.getElementById('notifier-type').value;
    const emailAddress = document.getElementById('email-address').value;
    const button = document.getElementById('add-notifier-btn');
    const responseBox = document.getElementById('notifiers-response');
    button.classList.add('btn-loading');
    
    if (!userId || !notifierName || !notifierType) {
        showModal('Please fill in all required fields');
        button.classList.remove('btn-loading');
        return;
    }
    
    const requestBody = {
        user_id: userId,
        notifier_name: notifierName,
        notifier_type: notifierType
    };
    
    // Add email address if it's an email notifier
    if (notifierType === 'ArxivEmailNotifier') {
        if (!emailAddress) {
            showModal('Please enter an email address for email notifier');
            button.classList.remove('btn-loading');
            return;
        }
        requestBody.email_address = emailAddress;
    }
    
    try {
        const response = await fetch(`${API_BASE_URL}/api/v1/notifiers`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify(requestBody)
        });
        
        // Check if the response is OK
        if (!response.ok) {
            throw new Error(`HTTP error! status: ${response.status}`);
        }
        
        // Try to parse as JSON
        const contentType = response.headers.get('content-type');
        if (contentType && contentType.includes('application/json')) {
            const data = await response.json();
            responseBox.textContent = JSON.stringify(data, null, 2);
        } else {
            const text = await response.text();
            responseBox.textContent = text || "Notifier added successfully.";
        }
        animateResponseBox(responseBox);
    } catch (error) {
        responseBox.textContent = `Error: ${error.message}`;
        animateResponseBox(responseBox);
    } finally {
        button.classList.remove('btn-loading');
    }
});

document.getElementById('delete-notifier-btn').addEventListener('click', async () => {
    const userId = document.getElementById('delete-notifier-user-id').value;
    const notifierName = document.getElementById('notifier-name-delete').value;
    const button = document.getElementById('delete-notifier-btn');
    const responseBox = document.getElementById('notifiers-response');
    button.classList.add('btn-loading');
    
    if (!userId || !notifierName) {
        showModal('Please enter both User ID and Notifier Name');
        button.classList.remove('btn-loading');
        return;
    }
    
    try {
        const response = await fetch(`${API_BASE_URL}/api/v1/notifiers/${userId}/${notifierName}`, {
            method: 'DELETE'
        });
        
        // Check if the response is OK
        if (!response.ok) {
            throw new Error(`HTTP error! status: ${response.status}`);
        }
        
        // Try to parse as JSON
        const contentType = response.headers.get('content-type');
        if (contentType && contentType.includes('application/json')) {
            const data = await response.json();
            responseBox.textContent = JSON.stringify(data, null, 2);
        } else {
            const text = await response.text();
            responseBox.textContent = text || "Notifier deleted successfully.";
        }
        animateResponseBox(responseBox);
    } catch (error) {
        responseBox.textContent = `Error: ${error.message}`;
        animateResponseBox(responseBox);
    } finally {
        button.classList.remove('btn-loading');
    }
});

// Add loading animation to CSS dynamically
const loadingStyle = document.createElement('style');
loadingStyle.innerHTML = `
    .btn-loading {
        position: relative;
        pointer-events: none;
        color: transparent !important;
    }
    
    .btn-loading::after {
        content: '';
        position: absolute;
        width: 16px;
        height: 16px;
        top: 50%;
        left: 50%;
        margin-top: -8px;
        margin-left: -8px;
        border: 2px solid transparent;
        border-top-color: #ffffff;
        border-radius: 50%;
        animation: btnLoadingSpinner 0.8s ease infinite;
    }
    
    @keyframes btnLoadingSpinner {
        from {
            transform: rotate(0turn);
        }
        to {
            transform: rotate(1turn);
        }
    }
`;
document.head.appendChild(loadingStyle);