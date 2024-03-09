
import './App.css'
import Form from './Form/Form';
import FilterButton from './FilterButton/FilterButton';
import TodoCard from './TodoCard/TodoCard';
import { useState } from 'react';

function App({data}) {
  
  const [tasks, setTasks] = useState(data);

  function addTask(title) {
    let id = tasks.length;
    const newTask = { id: id, title, completed: false };
    setTasks([...tasks, newTask]);
  }
  
  function toggleTaskCompleted(id) {
    const updatedTasks = tasks.map((task) => {
      if (id == task.id){
        return {...task,completed : !task.completed};
      }
      return task;
    });
    setTasks(updatedTasks);
  }

  function deleteTask(id){
    const updatedTasks = tasks.filter((task) => (id != task.id));
    setTasks(updatedTasks);
  }

  const taskList = tasks?.map((task) => (
    <TodoCard 
      id={task.id} 
      title={task.title} 
      completed={task.completed} 
      key={task.id} 
      toggleTaskCompleted={toggleTaskCompleted} 
      deleteTask={deleteTask} />
  )); 
  const tasksNoun = taskList.length !== 1 ? "tasks" : "task";
  const headingText = `${taskList.length} ${tasksNoun} remaining`;
  
  return (
    <div className="todoapp stack-large">
      <h1>TodoMatic</h1>
      <Form addTask={addTask}/>
      <div className="filters btn-group stack-exception">
        <FilterButton />
        <FilterButton />
        <FilterButton />
      </div>
      <h2 id="list-heading">{headingText}</h2>
      <ul
        role="list"
        className="todo-list stack-large stack-exception"
        aria-labelledby="list-heading">
        {taskList}
      </ul>
    </div>
  );
}



export default App;