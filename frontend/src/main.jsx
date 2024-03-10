import React from 'react'
import ReactDOM from 'react-dom/client'
import App from './App.jsx'
import './index.css'
import axios from 'axios'

async function fetch_todos(){
  const res = axios.get("http://localhost:8080/todos");
  return res;
}
var data = await fetch_todos();
var data = data.data == "None" ? [] : data.data;

ReactDOM.createRoot(document.getElementById('root')).render(
  <React.StrictMode>
    <App data={data} />
  </React.StrictMode>,
)
