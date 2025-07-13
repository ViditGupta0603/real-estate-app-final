import React, { useEffect, useState } from 'react';
import { useParams, useNavigate, useLocation } from 'react-router-dom';

const PropertyDetails = () => {
  const { id } = useParams();
  const navigate = useNavigate();
  const location = useLocation();

  // Try to get property from router state first
  const [property, setProperty] = useState(location.state?.property);

  // Local mock data as a fallback if you don't have an API or data file
  const mockProperties = [
    {
      id: "1",
      title: "Modern Apartment",
      imageUrl: "https://placehold.co/600x400",
      description: "A beautiful modern apartment in the city center.",
      price: "$350,000",
      location: "Downtown",
      features: ["2 Bedrooms", "2 Bathrooms", "Balcony", "Gym"],
      contact: "agent@example.com",
      fundedPercentage: 45
    },
    {
      id: "2",
      title: "Cozy Suburban House",
      imageUrl: "https://placehold.co/600x400",
      description: "Cozy family home in a quiet neighborhood.",
      price: "$500,000",
      location: "Suburbs",
      features: ["4 Bedrooms", "3 Bathrooms", "Garden", "Garage"],
      contact: "info@homes.com",
      fundedPercentage: 70
    }
  ];

  // Fallback: If property not in state, try to get it from the mock data
  useEffect(() => {
    if (!property) {
      const found = mockProperties.find(p => p.id === id);
      console.log("Loaded from mock:", found);
      setProperty(found);
    } else {
      console.log("Loaded from state:", property);
    }
    window.scrollTo(0, 0);
  }, [id, property]);

  if (!property) {
    return <div className="text-center text-gray-600">Property not found. Please go back.</div>;
  }

  return (
    <section className="bg-white p-6 rounded-lg shadow-md max-w-4xl mx-auto">
      <button
        onClick={() => navigate(-1)}
        className="mb-4 text-blue-600 hover:underline"
      >
        ← Back
      </button>

      <h2 className="text-3xl font-bold text-gray-800 mb-6">{property.title}</h2>

      <div className="flex flex-col md:flex-row gap-6 mb-6">
        <div className="md:w-2/3">
          <img
            src={property.imageUrl}
            alt={property.title}
            className="w-full h-64 sm:h-80 object-cover rounded-lg shadow-md"
            onError={(e) => {
              e.target.onerror = null;
              e.target.src = `https://placehold.co/600x400/C0C0C0/white?text=Image+Not+Found`;
            }}
          />
        </div>

        <div className="md:w-1/3 flex flex-col justify-between">
          <p className="text-gray-700 text-lg mb-4">{property.description}</p>

          <div className="mb-4">
            <p className="text-2xl font-semibold text-blue-600 mb-1">{property.price}</p>
            <p className="text-gray-600"><strong>Location:</strong> {property.location}</p>
          </div>

          {/* 🚫 Rocket Removed - Progress Bar Only */}
<div className="mb-6 relative max-w-md">
  <div className="text-sm text-gray-700 mb-1">
    Funded: <span className="font-semibold text-blue-600">{property.fundedPercentage || 0}%</span>
  </div>
  <div
    role="progressbar"
    aria-valuenow={property.fundedPercentage || 0}
    aria-valuemin={0}
    aria-valuemax={100}
    className="w-full bg-gray-200 rounded-full h-3 relative overflow-hidden"
  >
    <div
      className="bg-blue-500 h-3 rounded-full transition-all duration-500"
      style={{ width: `${property.fundedPercentage || 0}%` }}
    />
  </div>
</div>


          <h3 className="text-xl font-semibold text-gray-800 mb-2">Features</h3>
          <ul className="list-disc list-inside text-gray-700 mb-4">
            {property.features?.map((feature, i) => <li key={i}>{feature}</li>)}
          </ul>

          <div className="bg-blue-50 p-4 rounded-lg border border-blue-200">
            <h3 className="text-lg font-semibold text-blue-800 mb-2">Contact:</h3>
            <p className="text-blue-700">{property.contact || 'Contact agent for more details.'}</p>
          </div>
        </div>
      </div>
    </section>
  );
};

export default PropertyDetails;
