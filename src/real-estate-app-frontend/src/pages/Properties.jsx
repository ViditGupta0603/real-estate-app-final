import React, { useState } from "react";
import { Link } from "react-router-dom";
import { motion, AnimatePresence } from "framer-motion";
import { FaBed, FaBath, FaCar, FaTree } from "react-icons/fa";


const properties = [
  {
    id: "1",
    title: "Modern Apartment",
    imageUrl:
      "https://www.casagrand.co.in/wp-content/uploads/2025/04/SWIMMING-POOL-VIEW-scaled.jpg?ver=1.211",
    description: "A beautiful modern apartment in the city center.",
    price: "350,000 ICP",
    location: "Downtown",
    features: ["2 Bedrooms", "2 Bathrooms", "Balcony", "Gym"],
    contact: "agent@example.com",
    fundedPercentage: 70,
  },
  {
    id: "2",
    title: "Cozy Suburban House",
    imageUrl:
      "https://plus.unsplash.com/premium_photo-1746387628298-af5695a3f935?q=80&w=1332&auto=format&fit=crop&ixlib=rb-4.1.0&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
    description: "Cozy family home in a quiet neighborhood.",
    price: "500,000 ICP",
    location: "Suburbs",
    features: ["4 Bedrooms", "3 Bathrooms", "Garden", "Garage"],
    contact: "info@homes.com",
    fundedPercentage: 45,
  },
];

const container = {
  hidden: {},
  show: {
    transition: { staggerChildren: 0.13 },
  },
};

const card = {
  hidden: { opacity: 0, y: 32 },
  show: {
    opacity: 1,
    y: 0,
    transition: { type: "spring", bounce: 0.18, duration: 0.55 },
  },
};

const Properties = () => {
  const [sortOrder, setSortOrder] = useState("");

  const sortedProperties = [...properties].sort((a, b) => {
    const priceA = parseInt(a.price.replace(/[^0-9]/g, ""));
    const priceB = parseInt(b.price.replace(/[^0-9]/g, ""));
    if (sortOrder === "lowToHigh") return priceA - priceB;
    if (sortOrder === "highToLow") return priceB - priceA;
    return 0;
  });

  return (
    <motion.section
      className="min-h-screen bg-gradient-to-br from-blue-50 to-white pt-5 pb-20 px-6 sm:px-10"
      initial="hidden"
      animate="show"
      variants={container}
    >
      <div className="flex justify-end mb-6">
        <select
          value={sortOrder}
          onChange={(e) => setSortOrder(e.target.value)}
          className="border border-blue-300 rounded-md px-4 py-2 text-blue-800 focus:outline-none focus:ring-2 focus:ring-blue-400"
        >
          <option value="">Sort by</option>
          <option value="lowToHigh">Price: Low to High</option>
          <option value="highToLow">Price: High to Low</option>
        </select>
      </div>

      <div className="max-w-7xl mx-auto">
        <motion.h2
          className="text-4xl sm:text-5xl font-extrabold mb-4 text-center text-blue-900 tracking-tight"
          initial={{ opacity: 0, y: -20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ duration: 0.7 }}
        >
          Explore Our Properties
        </motion.h2>
        <motion.p
          className="text-center max-w-2xl mx-auto mb-16 text-gray-500 text-lg"
          initial={{ opacity: 0, y: -10 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ delay: 0.1, duration: 0.7 }}
        >
          Discover exceptional properties tailored to your lifestyle and budget.
        </motion.p>

        <motion.div
          className="grid gap-10 sm:grid-cols-2 lg:grid-cols-3"
          variants={container}
        >
          <AnimatePresence>
            {sortedProperties.map((property) => (
              <motion.div
                key={property.id}
                className="group bg-white/80 backdrop-blur-md rounded-3xl shadow-xl hover:shadow-2xl border border-gray-100 hover:border-blue-300 transition-all duration-300 flex flex-col overflow-hidden"
                variants={card}
                exit={{ opacity: 0, y: 32 }}
              >
                <div className="relative overflow-hidden">
                  <motion.img
                    src={property.imageUrl}
                    alt={property.title}
                    className="object-cover w-full h-60 transition-transform duration-300 group-hover:scale-105"
                  />
                  <div className="absolute inset-0 bg-black/20 opacity-0 group-hover:opacity-100 transition-opacity duration-300" />
                  <span className="absolute top-4 left-4 bg-blue-500 text-white px-4 py-1 rounded-full text-xs font-semibold shadow">
                    {property.location}
                  </span>
                </div>
                <div className="p-6 flex flex-col flex-1">
                  <h3 className="text-2xl font-bold text-blue-800 mb-2">{property.title}</h3>
                  <div className="text-lg text-blue-600 font-semibold mb-3">{property.price}</div>
                  <p className="text-gray-600 mb-4 flex-1">{property.description}</p>

                  <div className="flex flex-wrap gap-2 mb-4">
                    {property.features.map((feature, i) => (
                      <span
                        key={i}
                        className="bg-blue-50 border border-blue-100 text-blue-700 px-3 py-1 rounded-full text-xs font-medium"
                      >
                        {feature}
                      </span>
                    ))}
                  </div>

                  <motion.div whileHover={{ scale: 1.03 }} whileTap={{ scale: 0.97 }}>
                    <Link
                      to={`/properties/${property.id}`}
                      state={{ property }}
                      className="inline-block w-full rounded-xl bg-gradient-to-r from-blue-500 to-blue-600 hover:from-blue-600 hover:to-blue-700 text-white font-bold py-2 mt-auto text-center shadow-md transition-all duration-200"
                    >
                      View Details
                    </Link>
                  </motion.div>
                </div>
              </motion.div>
            ))}
          </AnimatePresence>
        </motion.div>

        <motion.div
          className="mt-24 text-center"
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ delay: 0.2, duration: 0.7 }}
        >
          <h4 className="text-lg font-semibold text-gray-700 mb-4">Looking for something more custom?</h4>
          <motion.div whileHover={{ scale: 1.05 }} whileTap={{ scale: 0.98 }} className="inline-block">
            <Link
              to="/contact"
              className="inline-block bg-gradient-to-r from-pink-500 to-red-500 hover:from-pink-600 hover:to-red-600 text-white px-8 py-3 rounded-full font-bold shadow-xl transition-all duration-200"
            >
              Contact Our Team
            </Link>
          </motion.div>
        </motion.div>
      </div>
    </motion.section>
  );
};

export default Properties;