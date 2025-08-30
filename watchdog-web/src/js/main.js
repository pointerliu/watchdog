// API Client for Watchdog
const API_BASE_URL = ''; // Use same origin (proxy through Express server)

// DOM Elements
const endpointItems = document.querySelectorAll('.endpoint-item');
const endpointDetails = document.querySelectorAll('.endpoint-details');

// Event Listeners for endpoint navigation
endpointItems.forEach(item => {
    item.addEventListener('click', () => {
        const endpoint = item.getAttribute('data-endpoint');
        
        // Update active state for sidebar items
        endpointItems.forEach(i => i.classList.remove('active'));
        item.classList.add('active');
        
        // Show the selected endpoint details
        endpointDetails.forEach(detail => {
            detail.classList.remove('active');
            if (detail.id === `${endpoint}-details`) {
                detail.classList.add('active');
            }
        });
    });
});

// Health Check
document.getElementById('health-check-btn').addEventListener('click', async () => {
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
        
        document.getElementById('health-response').textContent = JSON.stringify(data, null, 2);
    } catch (error) {
        document.getElementById('health-response').textContent = `Error: ${error.message}`;
    }
});

// Subscriptions
document.getElementById('get-subscriptions-btn').addEventListener('click', async () => {
    const userId = document.getElementById('get-user-id').value;
    if (!userId) {
        alert('Please enter a user ID');
        return;
    }
    
    try {
        const response = await fetch(`${API_BASE_URL}/api/v1/subscriptions/${userId}`, {
            method: 'GET'
        });
        
        // Handle case where user has no subscriptions (404)
        if (response.status === 404) {
            document.getElementById('subscriptions-response').textContent = "No subscriptions found for this user.";
            return;
        }
        
        // Check if the response is OK for other cases
        if (!response.ok) {
            throw new Error(`HTTP error! status: ${response.status}`);
        }
        
        // Handle empty response
        const text = await response.text();
        if (!text) {
            document.getElementById('subscriptions-response').textContent = "No subscriptions found for this user.";
            return;
        }
        
        // Try to parse as JSON
        const contentType = response.headers.get('content-type');
        if (contentType && contentType.includes('application/json')) {
            const data = JSON.parse(text);
            document.getElementById('subscriptions-response').textContent = JSON.stringify(data, null, 2);
        } else {
            document.getElementById('subscriptions-response').textContent = `Expected JSON but received: ${text.substring(0, 100)}...`;
        }
    } catch (error) {
        document.getElementById('subscriptions-response').textContent = `Error: ${error.message}`;
    }
});

document.getElementById('create-subscription-btn').addEventListener('click', async () => {
    const userId = document.getElementById('create-user-id').value;
    const subscriptionId = document.getElementById('subscription-id').value;
    const keywordsInput = document.getElementById('keywords').value;
    
    if (!userId || !subscriptionId) {
        alert('Please enter both User ID and Subscription ID');
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
            document.getElementById('subscriptions-response').textContent = JSON.stringify(data, null, 2);
        } else {
            const text = await response.text();
            document.getElementById('subscriptions-response').textContent = text || "Subscription created successfully.";
        }
    } catch (error) {
        document.getElementById('subscriptions-response').textContent = `Error: ${error.message}`;
    }
});

document.getElementById('delete-subscription-btn').addEventListener('click', async () => {
    const userId = document.getElementById('delete-user-id').value;
    const subscriptionId = document.getElementById('delete-subscription-id').value;
    
    if (!userId || !subscriptionId) {
        alert('Please enter both User ID and Subscription ID');
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
            document.getElementById('subscriptions-response').textContent = JSON.stringify(data, null, 2);
        } else {
            const text = await response.text();
            document.getElementById('subscriptions-response').textContent = text || "Subscription deleted successfully.";
        }
    } catch (error) {
        document.getElementById('subscriptions-response').textContent = `Error: ${error.message}`;
    }
});

// Fetchers
document.getElementById('get-fetcher-types-btn').addEventListener('click', async () => {
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
            document.getElementById('fetchers-response').textContent = JSON.stringify(data, null, 2);
        } else {
            const text = await response.text();
            document.getElementById('fetchers-response').textContent = text || "No fetcher types found.";
        }
    } catch (error) {
        document.getElementById('fetchers-response').textContent = `Error: ${error.message}`;
    }
});

document.getElementById('get-fetchers-btn').addEventListener('click', async () => {
    const userId = document.getElementById('get-fetchers-user-id').value;
    if (!userId) {
        alert('Please enter a user ID');
        return;
    }
    
    try {
        const response = await fetch(`${API_BASE_URL}/api/v1/fetchers/${userId}`, {
            method: 'GET'
        });
        
        // Handle case where user has no fetchers (404)
        if (response.status === 404) {
            document.getElementById('fetchers-response').textContent = "No fetchers found for this user.";
            return;
        }
        
        // Check if the response is OK for other cases
        if (!response.ok) {
            throw new Error(`HTTP error! status: ${response.status}`);
        }
        
        // Handle empty response
        const text = await response.text();
        if (!text) {
            document.getElementById('fetchers-response').textContent = "No fetchers found for this user.";
            return;
        }
        
        // Try to parse as JSON
        const contentType = response.headers.get('content-type');
        if (contentType && contentType.includes('application/json')) {
            const data = JSON.parse(text);
            document.getElementById('fetchers-response').textContent = JSON.stringify(data, null, 2);
        } else {
            document.getElementById('fetchers-response').textContent = `Expected JSON but received: ${text.substring(0, 100)}...`;
        }
    } catch (error) {
        document.getElementById('fetchers-response').textContent = `Error: ${error.message}`;
    }
});

document.getElementById('add-fetcher-btn').addEventListener('click', async () => {
    const userId = document.getElementById('add-fetcher-user-id').value;
    const fetcherName = document.getElementById('fetcher-name').value;
    const fetcherType = document.getElementById('fetcher-type').value;
    const subscriptionId = document.getElementById('fetcher-subscription-id').value;
    
    if (!userId || !fetcherName || !fetcherType || !subscriptionId) {
        alert('Please fill in all fields');
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
            document.getElementById('fetchers-response').textContent = JSON.stringify(data, null, 2);
        } else {
            const text = await response.text();
            document.getElementById('fetchers-response').textContent = text || "Fetcher added successfully.";
        }
    } catch (error) {
        document.getElementById('fetchers-response').textContent = `Error: ${error.message}`;
    }
});

document.getElementById('delete-fetcher-btn').addEventListener('click', async () => {
    const userId = document.getElementById('delete-fetcher-user-id').value;
    const fetcherName = document.getElementById('fetcher-name-delete').value;
    
    if (!userId || !fetcherName) {
        alert('Please enter both User ID and Fetcher Name');
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
            document.getElementById('fetchers-response').textContent = JSON.stringify(data, null, 2);
        } else {
            const text = await response.text();
            document.getElementById('fetchers-response').textContent = text || "Fetcher deleted successfully.";
        }
    } catch (error) {
        document.getElementById('fetchers-response').textContent = `Error: ${error.message}`;
    }
});

// Notifiers
// Toggle email field based on notifier type
document.getElementById('notifier-type').addEventListener('change', function() {
    const emailField = document.getElementById('email-field');
    if (this.value === 'ArxivEmailNotifier') {
        emailField.style.display = 'block';
    } else {
        emailField.style.display = 'none';
    }
});

document.getElementById('get-notifier-types-btn').addEventListener('click', async () => {
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
            document.getElementById('notifiers-response').textContent = JSON.stringify(data, null, 2);
        } else {
            const text = await response.text();
            document.getElementById('notifiers-response').textContent = text || "No notifier types found.";
        }
    } catch (error) {
        document.getElementById('notifiers-response').textContent = `Error: ${error.message}`;
    }
});

document.getElementById('get-notifiers-btn').addEventListener('click', async () => {
    const userId = document.getElementById('get-notifiers-user-id').value;
    if (!userId) {
        alert('Please enter a user ID');
        return;
    }
    
    try {
        const response = await fetch(`${API_BASE_URL}/api/v1/notifiers/${userId}`, {
            method: 'GET'
        });
        
        // Handle case where user has no notifiers (404)
        if (response.status === 404) {
            document.getElementById('notifiers-response').textContent = "No notifiers found for this user.";
            return;
        }
        
        // Check if the response is OK for other cases
        if (!response.ok) {
            throw new Error(`HTTP error! status: ${response.status}`);
        }
        
        // Handle empty response
        const text = await response.text();
        if (!text) {
            document.getElementById('notifiers-response').textContent = "No notifiers found for this user.";
            return;
        }
        
        // Try to parse as JSON
        const contentType = response.headers.get('content-type');
        if (contentType && contentType.includes('application/json')) {
            const data = JSON.parse(text);
            document.getElementById('notifiers-response').textContent = JSON.stringify(data, null, 2);
        } else {
            document.getElementById('notifiers-response').textContent = `Expected JSON but received: ${text.substring(0, 100)}...`;
        }
    } catch (error) {
        document.getElementById('notifiers-response').textContent = `Error: ${error.message}`;
    }
});

document.getElementById('add-notifier-btn').addEventListener('click', async () => {
    const userId = document.getElementById('add-notifier-user-id').value;
    const notifierName = document.getElementById('notifier-name').value;
    const notifierType = document.getElementById('notifier-type').value;
    const emailAddress = document.getElementById('email-address').value;
    
    if (!userId || !notifierName || !notifierType) {
        alert('Please fill in all required fields');
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
            alert('Please enter an email address for email notifier');
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
            document.getElementById('notifiers-response').textContent = JSON.stringify(data, null, 2);
        } else {
            const text = await response.text();
            document.getElementById('notifiers-response').textContent = text || "Notifier added successfully.";
        }
    } catch (error) {
        document.getElementById('notifiers-response').textContent = `Error: ${error.message}`;
    }
});

document.getElementById('delete-notifier-btn').addEventListener('click', async () => {
    const userId = document.getElementById('delete-notifier-user-id').value;
    const notifierName = document.getElementById('notifier-name-delete').value;
    
    if (!userId || !notifierName) {
        alert('Please enter both User ID and Notifier Name');
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
            document.getElementById('notifiers-response').textContent = JSON.stringify(data, null, 2);
        } else {
            const text = await response.text();
            document.getElementById('notifiers-response').textContent = text || "Notifier deleted successfully.";
        }
    } catch (error) {
        document.getElementById('notifiers-response').textContent = `Error: ${error.message}`;
    }
});