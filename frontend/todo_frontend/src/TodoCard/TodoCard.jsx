import "./TodoCard.css"
function TodoCard({id,title,completed,toggleTaskCompleted,deleteTask}){
    return(
        <li className="todo stack-small">
        <div className="c-cb">
          <input id={"todo_id_"+id} type="checkbox" defaultChecked={completed} onChange={() => toggleTaskCompleted(id)} />
          <label className="todo-label" htmlFor={"todo_id_"+id}>
            {title}
          </label>
        </div>
        <div className="btn-group">
          <button type="button" className="btn">
            Edit <span className="visually-hidden">{title}</span>
          </button>
          <button type="button" className="btn btn__danger" onClick={() => deleteTask(id)}>
            Delete <span className="visually-hidden" >{title}</span>
          </button>
        </div>
      </li>
    );
}

export default TodoCard

