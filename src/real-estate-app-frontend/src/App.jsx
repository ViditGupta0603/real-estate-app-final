import React from "react";
import { BrowserRouter as Router, Route, Routes } from "react-router-dom";
import Home from "./pages/Home";
import Navbar from "./components/Navbar";
import Properties from "./pages/Properties"; // import the new landing page
import PropertyDetails from "./pages/PropertyDetails";
import Portfolio from "./pages/Portfolio";

const App = () => {
  return (
    <Router>
      <Navbar />
      <Routes>
        <Route path="/" element={<Home />} />
        <Route path="/properties" element={<Properties />} /> {/* Landing page */}
        <Route path="/properties/:id" element={<PropertyDetails />} /> {/* Details page */}
        <Route path="/portfolio" element={<Portfolio />} />
      </Routes>
    </Router>
  );
};

export default App;