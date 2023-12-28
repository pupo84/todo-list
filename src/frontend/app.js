document.addEventListener('DOMContentLoaded', function() {
    const todoForm = document.getElementById('todoForm');
    const todoInput = document.getElementById('todoInput');
    const todoList = document.getElementById('todoList');

    // Function to fetch todos from the API and render them
    function fetchAndRenderTodos() {
        fetch('http://localhost:8080/api/todos')
            .then(response => response.json())
            .then(todos => {
                todoList.innerHTML = '';
                todos.forEach(renderTodo);
            })
            .catch(error => console.error('Error fetching todos:', error));
    }

    // Function to render a single todo item
    function renderTodo(todo) {
        const li = document.createElement('li');
        li.innerHTML = `
            <span class="${todo.completed ? 'completed' : ''}">${todo.title}</span>
            <button class="delete-btn" onclick="deleteTodo('${todo.id}')">Delete</button>
        `;
        todoList.appendChild(li);
    }

    // Function to add a new todo
    function addTodo(title) {
        fetch('http://localhost:8080/api/todos', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({ title }),
        })
            .then(response => response.json())
            .then(newTodo => {
                renderTodo(newTodo);
            })
            .catch(error => console.error('Error adding todo:', error));
    }

    // Function to delete a todo
    window.deleteTodo = function(id) {
        fetch(`http://localhost:8080/api/todos/${id}`, {
            method: 'DELETE',
        })
            .then(response => {
                if (response.ok) {
                    fetchAndRenderTodos();
                } else {
                    console.error('Error deleting todo:', response.statusText);
                }
            })
            .catch(error => console.error('Error deleting todo:', error));
    };

    // Event listener for the todo form submission
    todoForm.addEventListener('submit', function(event) {
        event.preventDefault();
        const newTodoTitle = todoInput.value.trim();
        if (newTodoTitle) {
            addTodo(newTodoTitle);
            todoInput.value = '';
        }
    });

    // Fetch and render todos on page load
    fetchAndRenderTodos();
});