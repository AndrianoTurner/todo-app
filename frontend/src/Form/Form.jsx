import { useState } from "react";
function Form({addTask}){

    const [title, setTitle] = useState("");
    function handleChange(event) {
        var title = event.target.value;
        if (title == undefined){
            title = "Not set";
        }
        setTitle(title);
    };

    function handleSubmit(event) {
        event.preventDefault();
        addTask(title);
        setTitle("");
    };
      
    return (
        <>
        <form onSubmit={handleSubmit}>
            <h2 className="label-wrapper">
            <label htmlFor="new-todo-input" className="label__lg">
                What needs to be done?
            </label>
            </h2>
            <input
                type="text"
                id="new-todo-input"
                className="input input__lg"
                name="text"
                autoComplete="off"
                value={title}
                onChange={handleChange}
            />
            <button type="submit" className="btn btn__primary btn__lg">
                Add
            </button>
        </form>
        </>
    );
}

export default Form;