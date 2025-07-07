// import { useState } from 'react';
// import { real_estate_app_backend } from 'declarations/real-estate-app-backend';

// function App() {
//   const [greeting, setGreeting] = useState('');

//   function handleSubmit(event) {
//     event.preventDefault();
//     const name = event.target.elements.name.value;
//     real_estate_app_backend.greet(name).then((greeting) => {
//       setGreeting(greeting);
//     });
//     return false;
//   }

//   return (
//     <main>
//       <img src="/logo2.svg" alt="DFINITY logo" />
//       <br />
//       <br />
//       <form action="#" onSubmit={handleSubmit}>
//         <label htmlFor="name">Enter your name: &nbsp;</label>
//         <input id="name" alt="Name" type="text" />
//         <button type="submit">Click Me!</button>
//       </form>
//       <section id="greeting">{greeting}</section>
//     </main>
//   );
// }

// export default App;


import React from 'react'

const App = () => {
  return (
    <div className='text-3xl'>App</div>
  )
}

export default App