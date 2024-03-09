
import { useState } from 'react';
import TodoCard from '../TodoCard/TodoCard';

function TodoCardList(){
    const [todos, setTodos] = useState([]);

    const addTodo = (title,description,completed) =>{
        todos.push(
            {
                title : title,
                description : description,
                completed : completed
            }
        )
        setTodos(todos)
    }
    const deleteTodo = (title) => {
        setTodos(todos.filter((item) => item.title !== title));
    };

    const setCompleted = (title) => {
        setTodos(
          todos.map((item) =>
            item.title === title ? { ...item, completed: !item.completed } : item
          )
        );
      };
    todos.push(
        {
            title : "Test",
            description : "123",
            completed : false
        }
    )
    
    const [newTodo,setNewTodo] = useState({
        title : "Title...",
        description : "Description",
        completed : false,
    })
    return (
        <>
        <ul>
          {todos.map(item => 
          <li key={item.title}>
            <TodoCard title={item.title} description={item.description} completed={item.completed}></TodoCard>
          </li> )}

            <TodoCard title={newTodo.title} description={newTodo.description} completed={newTodo.completed}></TodoCard>
          <button type='button' onClick={addTodo}>Add todo</button>
        </ul>
        </>
    );
}

export default TodoCardList