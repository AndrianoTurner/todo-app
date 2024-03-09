import React from 'react'
import ReactDOM from 'react-dom/client'
import App from './App.jsx'
import './index.css'
const DATA = [
  { id: 0, title: "Eat", completed: true },
  { id: 1, title: "Sleep", completed: false },
  { id: 2, title: "Repeat", completed: false },
];
ReactDOM.createRoot(document.getElementById('root')).render(
  <React.StrictMode>
    <App data={DATA} />
  </React.StrictMode>,
)
