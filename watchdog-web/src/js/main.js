// API Client for Watchdog
const API_BASE_URL = ''; // Use same origin (proxy through Express server)

// DOM Elements
const endpointItems = document.querySelectorAll('.endpoint-item');
const endpointDetails = document.querySelectorAll('.endpoint-details');
const darkModeToggle = document.getElementById('dark-mode-toggle');
const darkModeIcon = document.querySelector('#dark-mode-toggle i');

// Custom Modal Elements
const modal = document.getElementById('custom-modal');
const modalMessage = document.getElementById('modal-message');
const closeModalBtn = document.querySelector('.close-modal');
const modalOkBtn = document.getElementById('modal-ok-btn');

// Check for saved dark mode preference or default to light mode
function initDarkMode() {
    const isDarkMode = localStorage.getItem('darkMode') === 'true';
    if (isDarkMode) {
        document.documentElement.classList.add('dark-mode');
        darkModeIcon.classList.remove('fa-moon');
        darkModeIcon.classList.add('fa-sun');
    }
}

// Toggle dark mode
function toggleDarkMode() {
    const isDarkMode = document.documentElement.classList.toggle('dark-mode');
    localStorage.setItem('darkMode', isDarkMode);
    
    // Update icon
    if (isDarkMode) {
        darkModeIcon.classList.remove('fa-moon');
        darkModeIcon.classList.add('fa-sun');
    } else {
        darkModeIcon.classList.remove('fa-sun');
        darkModeIcon.classList.add('fa-moon');
    }
}

// Initialize dark mode on page load
initDarkMode();

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

// Utility function to get user ID from input field
function getUserId(section) {
    const input = document.getElementById(`${section}-user-id`);
    return input ? input.value.trim() : '';
}

// Utility function to show/hide form sections
function toggleForm(section, show) {
    const form = document.getElementById(`add-${section}-form`);
    if (form) {
        if (show) {
            form.classList.add('active');
        } else {
            form.classList.remove('active');
        }
    }
}

// Utility function to clear form inputs
function clearFormInputs(section) {
    const inputs = document.querySelectorAll(`#add-${section}-form input, #add-${section}-form select`);
    inputs.forEach(input => {
        if (input.type === 'checkbox') {
            input.checked = false;
        } else {
            input.value = '';
        }
    });
}

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
document.getElementById('load-subscriptions-btn').addEventListener('click', async () => {
    const userId = getUserId('subscriptions');
    const button = document.getElementById('load-subscriptions-btn');
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
            renderSubscriptionsTable(userId, []);
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
            renderSubscriptionsTable(userId, []);
            responseBox.textContent = "No subscriptions found for this user.";
            animateResponseBox(responseBox);
            return;
        }
        
        // Try to parse as JSON
        const contentType = response.headers.get('content-type');
        if (contentType && contentType.includes('application/json')) {
            const data = JSON.parse(text);
            // Handle the response format: { status: 200, message: "success", data: [...] }
            let subscriptions = [];
            if (Array.isArray(data.data)) {
                // If data.data is an array of subscription IDs
                subscriptions = data.data.map(id => ({ subscription_id: id }));
            } else if (Array.isArray(data)) {
                // If data is directly an array
                subscriptions = data.map(item => 
                    typeof item === 'string' ? { subscription_id: item } : item
                );
            }
            renderSubscriptionsTable(userId, subscriptions);
            responseBox.textContent = JSON.stringify(data, null, 2);
        } else {
            renderSubscriptionsTable(userId, []);
            responseBox.textContent = `Expected JSON but received: ${text.substring(0, 100)}...`;
        }
        animateResponseBox(responseBox);
    } catch (error) {
        renderSubscriptionsTable(userId, []);
        responseBox.textContent = `Error: ${error.message}`;
        animateResponseBox(responseBox);
    } finally {
        button.classList.remove('btn-loading');
    }
});

function renderSubscriptionsTable(userId, subscriptions) {
    const tableBody = document.querySelector('#subscriptions-table tbody');
    const emptyMessage = document.getElementById('subscriptions-empty');
    
    // Clear existing rows
    tableBody.innerHTML = '';
    
    if (subscriptions && subscriptions.length > 0) {
        emptyMessage.style.display = 'none';
        
        subscriptions.forEach(subscription => {
            const row = document.createElement('tr');
            
            // Subscription ID cell
            const idCell = document.createElement('td');
            idCell.textContent = subscription.subscription_id || subscription.id || 'N/A';
            row.appendChild(idCell);
            
            // Actions cell
            const actionsCell = document.createElement('td');
            actionsCell.className = 'action-cell';
            
            const deleteButton = document.createElement('button');
            deleteButton.className = 'btn danger action-btn';
            deleteButton.innerHTML = '<i class="fas fa-trash"></i> Remove';
            deleteButton.addEventListener('click', () => {
                deleteSubscription(userId, subscription.subscription_id || subscription.id);
            });
            
            actionsCell.appendChild(deleteButton);
            row.appendChild(actionsCell);
            
            tableBody.appendChild(row);
        });
    } else {
        emptyMessage.style.display = 'block';
    }
}

async function deleteSubscription(userId, subscriptionId) {
    const responseBox = document.getElementById('subscriptions-response');
    
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
        let message = "Subscription deleted successfully.";
        if (contentType && contentType.includes('application/json')) {
            const data = await response.json();
            message = JSON.stringify(data, null, 2);
        } else {
            const text = await response.text();
            message = text || "Subscription deleted successfully.";
        }
        
        responseBox.textContent = message;
        animateResponseBox(responseBox);
        
        // Reload the subscriptions
        document.getElementById('load-subscriptions-btn').click();
    } catch (error) {
        responseBox.textContent = `Error: ${error.message}`;
        animateResponseBox(responseBox);
    }
}

document.getElementById('add-subscription-btn').addEventListener('click', () => {
    toggleForm('subscription', true);
});

document.getElementById('cancel-subscription-btn').addEventListener('click', () => {
    toggleForm('subscription', false);
    clearFormInputs('subscription');
});

document.getElementById('create-subscription-btn').addEventListener('click', async () => {
    const userId = getUserId('subscriptions');
    const subscriptionId = document.getElementById('new-subscription-id').value;
    const keywordsInput = document.getElementById('new-keywords').value;
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
        
        // Hide form and clear inputs
        toggleForm('subscription', false);
        clearFormInputs('subscription');
        
        // Reload the subscriptions
        document.getElementById('load-subscriptions-btn').click();
    } catch (error) {
        responseBox.textContent = `Error: ${error.message}`;
        animateResponseBox(responseBox);
    } finally {
        button.classList.remove('btn-loading');
    }
});

// Fetchers
document.getElementById('load-fetchers-btn').addEventListener('click', async () => {
    const userId = getUserId('fetchers');
    const button = document.getElementById('load-fetchers-btn');
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
            renderFetchersTable(userId, []);
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
            renderFetchersTable(userId, []);
            responseBox.textContent = "No fetchers found for this user.";
            animateResponseBox(responseBox);
            return;
        }
        
        // Try to parse as JSON
        const contentType = response.headers.get('content-type');
        if (contentType && contentType.includes('application/json')) {
            const data = JSON.parse(text);
            // Handle the response format: { status: 200, message: "success", data: [...] }
            let fetchers = [];
            if (Array.isArray(data.data)) {
                // If data.data is an array of fetcher names
                fetchers = data.data.map(name => ({ fetcher_name: name }));
            } else if (Array.isArray(data)) {
                // If data is directly an array
                fetchers = data.map(item => 
                    typeof item === 'string' ? { fetcher_name: item } : item
                );
            }
            renderFetchersTable(userId, fetchers);
            responseBox.textContent = JSON.stringify(data, null, 2);
        } else {
            renderFetchersTable(userId, []);
            responseBox.textContent = `Expected JSON but received: ${text.substring(0, 100)}...`;
        }
        animateResponseBox(responseBox);
    } catch (error) {
        renderFetchersTable(userId, []);
        responseBox.textContent = `Error: ${error.message}`;
        animateResponseBox(responseBox);
    } finally {
        button.classList.remove('btn-loading');
    }
});

function renderFetchersTable(userId, fetchers) {
    const tableBody = document.querySelector('#fetchers-table tbody');
    const emptyMessage = document.getElementById('fetchers-empty');
    
    // Clear existing rows
    tableBody.innerHTML = '';
    
    if (fetchers && fetchers.length > 0) {
        emptyMessage.style.display = 'none';
        
        fetchers.forEach(fetcher => {
            const row = document.createElement('tr');
            
            // Fetcher Name cell
            const nameCell = document.createElement('td');
            nameCell.textContent = fetcher.fetcher_name || fetcher.name || 'N/A';
            row.appendChild(nameCell);
            
            // Actions cell
            const actionsCell = document.createElement('td');
            actionsCell.className = 'action-cell';
            
            const deleteButton = document.createElement('button');
            deleteButton.className = 'btn danger action-btn';
            deleteButton.innerHTML = '<i class="fas fa-trash"></i> Remove';
            deleteButton.addEventListener('click', () => {
                deleteFetcher(userId, fetcher.fetcher_name || fetcher.name);
            });
            
            actionsCell.appendChild(deleteButton);
            row.appendChild(actionsCell);
            
            tableBody.appendChild(row);
        });
    } else {
        emptyMessage.style.display = 'block';
    }
}

async function deleteFetcher(userId, fetcherName) {
    const responseBox = document.getElementById('fetchers-response');
    
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
        let message = "Fetcher deleted successfully.";
        if (contentType && contentType.includes('application/json')) {
            const data = await response.json();
            message = JSON.stringify(data, null, 2);
        } else {
            const text = await response.text();
            message = text || "Fetcher deleted successfully.";
        }
        
        responseBox.textContent = message;
        animateResponseBox(responseBox);
        
        // Reload the fetchers
        document.getElementById('load-fetchers-btn').click();
    } catch (error) {
        responseBox.textContent = `Error: ${error.message}`;
        animateResponseBox(responseBox);
    }
}

document.getElementById('add-fetcher-btn').addEventListener('click', () => {
    toggleForm('fetcher', true);
});

document.getElementById('cancel-fetcher-btn').addEventListener('click', () => {
    toggleForm('fetcher', false);
    clearFormInputs('fetcher');
});

document.getElementById('create-fetcher-btn').addEventListener('click', async () => {
    const userId = getUserId('fetchers');
    const fetcherName = document.getElementById('new-fetcher-name').value;
    const fetcherType = document.getElementById('new-fetcher-type').value;
    const subscriptionId = document.getElementById('new-fetcher-subscription-id').value;
    const button = document.getElementById('create-fetcher-btn');
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
        
        // Hide form and clear inputs
        toggleForm('fetcher', false);
        clearFormInputs('fetcher');
        
        // Reload the fetchers
        document.getElementById('load-fetchers-btn').click();
    } catch (error) {
        responseBox.textContent = `Error: ${error.message}`;
        animateResponseBox(responseBox);
    } finally {
        button.classList.remove('btn-loading');
    }
});

// Notifiers
// Toggle email field based on notifier type
document.getElementById('new-notifier-type').addEventListener('change', function() {
    const emailField = document.getElementById('new-email-field');
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

document.getElementById('load-notifiers-btn').addEventListener('click', async () => {
    const userId = getUserId('notifiers');
    const button = document.getElementById('load-notifiers-btn');
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
            renderNotifiersTable(userId, []);
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
            renderNotifiersTable(userId, []);
            responseBox.textContent = "No notifiers found for this user.";
            animateResponseBox(responseBox);
            return;
        }
        
        // Try to parse as JSON
        const contentType = response.headers.get('content-type');
        if (contentType && contentType.includes('application/json')) {
            const data = JSON.parse(text);
            // Handle the response format: { status: 200, message: "success", data: [...] }
            let notifiers = [];
            if (Array.isArray(data.data)) {
                // If data.data is an array of notifier names
                notifiers = data.data.map(name => ({ notifier_name: name }));
            } else if (Array.isArray(data)) {
                // If data is directly an array
                notifiers = data.map(item => 
                    typeof item === 'string' ? { notifier_name: item } : item
                );
            }
            renderNotifiersTable(userId, notifiers);
            responseBox.textContent = JSON.stringify(data, null, 2);
        } else {
            renderNotifiersTable(userId, []);
            responseBox.textContent = `Expected JSON but received: ${text.substring(0, 100)}...`;
        }
        animateResponseBox(responseBox);
    } catch (error) {
        renderNotifiersTable(userId, []);
        responseBox.textContent = `Error: ${error.message}`;
        animateResponseBox(responseBox);
    } finally {
        button.classList.remove('btn-loading');
    }
});

function renderNotifiersTable(userId, notifiers) {
    const tableBody = document.querySelector('#notifiers-table tbody');
    const emptyMessage = document.getElementById('notifiers-empty');
    
    // Clear existing rows
    tableBody.innerHTML = '';
    
    if (notifiers && notifiers.length > 0) {
        emptyMessage.style.display = 'none';
        
        notifiers.forEach(notifier => {
            const row = document.createElement('tr');
            
            // Notifier Name cell
            const nameCell = document.createElement('td');
            nameCell.textContent = notifier.notifier_name || notifier.name || 'N/A';
            row.appendChild(nameCell);
            
            // Actions cell
            const actionsCell = document.createElement('td');
            actionsCell.className = 'action-cell';
            
            const deleteButton = document.createElement('button');
            deleteButton.className = 'btn danger action-btn';
            deleteButton.innerHTML = '<i class="fas fa-trash"></i> Remove';
            deleteButton.addEventListener('click', () => {
                deleteNotifier(userId, notifier.notifier_name || notifier.name);
            });
            
            actionsCell.appendChild(deleteButton);
            row.appendChild(actionsCell);
            
            tableBody.appendChild(row);
        });
    } else {
        emptyMessage.style.display = 'block';
    }
}

async function deleteNotifier(userId, notifierName) {
    const responseBox = document.getElementById('notifiers-response');
    
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
        let message = "Notifier deleted successfully.";
        if (contentType && contentType.includes('application/json')) {
            const data = await response.json();
            message = JSON.stringify(data, null, 2);
        } else {
            const text = await response.text();
            message = text || "Notifier deleted successfully.";
        }
        
        responseBox.textContent = message;
        animateResponseBox(responseBox);
        
        // Reload the notifiers
        document.getElementById('load-notifiers-btn').click();
    } catch (error) {
        responseBox.textContent = `Error: ${error.message}`;
        animateResponseBox(responseBox);
    }
}

document.getElementById('add-notifier-btn').addEventListener('click', () => {
    toggleForm('notifier', true);
});

document.getElementById('cancel-notifier-btn').addEventListener('click', () => {
    toggleForm('notifier', false);
    clearFormInputs('notifier');
});

document.getElementById('create-notifier-btn').addEventListener('click', async () => {
    const userId = getUserId('notifiers');
    const notifierName = document.getElementById('new-notifier-name').value;
    const notifierType = document.getElementById('new-notifier-type').value;
    const emailAddress = document.getElementById('new-email-address').value;
    const button = document.getElementById('create-notifier-btn');
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
        
        // Hide form and clear inputs
        toggleForm('notifier', false);
        clearFormInputs('notifier');
        
        // Reload the notifiers
        document.getElementById('load-notifiers-btn').click();
    } catch (error) {
        responseBox.textContent = `Error: ${error.message}`;
        animateResponseBox(responseBox);
    } finally {
        button.classList.remove('btn-loading');
    }
});

// Add event listener to dark mode toggle button
if (darkModeToggle) {
    darkModeToggle.addEventListener('click', toggleDarkMode);
}