export function normalize(value) {
  return String(value ?? '')
    .normalize('NFD')
    .replace(/[\u0300-\u036f]/g, '')
    .toLowerCase()
    .replace(/[.,'’()-]/g, '')
    .replace(/[&_]/g, ' ')
    .replace(/\s+/g, ' ')
    .trim();
}

const NAME_ALIASES = {
  'new york city': 'new york',
  nyc: 'new york',
  bombay: 'mumbai',
  calcutta: 'kolkata',
  peking: 'beijing',
  saigon: 'ho chi minh city',
  'mexico df': 'mexico city',
  cdmx: 'mexico city',
  'washington dc': 'washington',
  'washington d c': 'washington',
};

const COUNTRY_ALIASES = {
  uk: 'united kingdom',
  gb: 'united kingdom',
  'great britain': 'united kingdom',
  england: 'united kingdom',
  usa: 'united states',
  us: 'united states',
  america: 'united states',
  'united states of america': 'united states',
};

export function normalizeCountry(country) {
  const n = normalize(country);
  return COUNTRY_ALIASES[n] ?? n;
}

// [name, country, latitude, longitude]; most prominent entry first for duplicate names.
const RAW = [
  ['London', 'United Kingdom', 51.5074, -0.1278],
  ['New York', 'United States', 40.7128, -74.006],
  ['Tokyo', 'Japan', 35.6762, 139.6503],
  ['Sydney', 'Australia', -33.8688, 151.2093],
  ['Paris', 'France', 48.8566, 2.3522],
  ['Berlin', 'Germany', 52.52, 13.405],
  ['Mumbai', 'India', 19.076, 72.8777],
  ['São Paulo', 'Brazil', -23.5505, -46.6333],
  ['Cape Town', 'South Africa', -33.9249, 18.4241],
  ['Toronto', 'Canada', 43.6532, -79.3832],
  ['Paris', 'United States', 33.6609, -95.5555],
  ['London', 'Canada', 42.9849, -81.2453],
  ['Los Angeles', 'United States', 34.0522, -118.2437],
  ['Chicago', 'United States', 41.8781, -87.6298],
  ['San Francisco', 'United States', 37.7749, -122.4194],
  ['Seattle', 'United States', 47.6062, -122.3321],
  ['Miami', 'United States', 25.7617, -80.1918],
  ['Houston', 'United States', 29.7604, -95.3698],
  ['Boston', 'United States', 42.3601, -71.0589],
  ['Washington', 'United States', 38.9072, -77.0369],
  ['Denver', 'United States', 39.7392, -104.9903],
  ['Phoenix', 'United States', 33.4484, -112.074],
  ['Las Vegas', 'United States', 36.1699, -115.1398],
  ['Atlanta', 'United States', 33.749, -84.388],
  ['Dallas', 'United States', 32.7767, -96.797],
  ['Anchorage', 'United States', 61.2181, -149.9003],
  ['Honolulu', 'United States', 21.3099, -157.8581],
  ['Vancouver', 'Canada', 49.2827, -123.1207],
  ['Montreal', 'Canada', 45.5017, -73.5673],
  ['Calgary', 'Canada', 51.0447, -114.0719],
  ['Ottawa', 'Canada', 45.4215, -75.6972],
  ['Mexico City', 'Mexico', 19.4326, -99.1332],
  ['Guadalajara', 'Mexico', 20.6597, -103.3496],
  ['Havana', 'Cuba', 23.1136, -82.3666],
  ['Panama City', 'Panama', 8.9824, -79.5199],
  ['San José', 'Costa Rica', 9.9281, -84.0907],
  ['Bogotá', 'Colombia', 4.711, -74.0721],
  ['Medellín', 'Colombia', 6.2476, -75.5658],
  ['Lima', 'Peru', -12.0464, -77.0428],
  ['Quito', 'Ecuador', -0.1807, -78.4678],
  ['Caracas', 'Venezuela', 10.4806, -66.9036],
  ['Santiago', 'Chile', -33.4489, -70.6693],
  ['Buenos Aires', 'Argentina', -34.6037, -58.3816],
  ['Montevideo', 'Uruguay', -34.9011, -56.1645],
  ['La Paz', 'Bolivia', -16.4897, -68.1193],
  ['Rio de Janeiro', 'Brazil', -22.9068, -43.1729],
  ['Brasília', 'Brazil', -15.7939, -47.8828],
  ['Salvador', 'Brazil', -12.9777, -38.5016],
  ['Manaus', 'Brazil', -3.119, -60.0217],
  ['Reykjavik', 'Iceland', 64.1466, -21.9426],
  ['Dublin', 'Ireland', 53.3498, -6.2603],
  ['Edinburgh', 'United Kingdom', 55.9533, -3.1883],
  ['Manchester', 'United Kingdom', 53.4808, -2.2426],
  ['Glasgow', 'United Kingdom', 55.8642, -4.2518],
  ['Madrid', 'Spain', 40.4168, -3.7038],
  ['Barcelona', 'Spain', 41.3874, 2.1686],
  ['Lisbon', 'Portugal', 38.7223, -9.1393],
  ['Marseille', 'France', 43.2965, 5.3698],
  ['Lyon', 'France', 45.764, 4.8357],
  ['Amsterdam', 'Netherlands', 52.3676, 4.9041],
  ['Brussels', 'Belgium', 50.8503, 4.3517],
  ['Munich', 'Germany', 48.1351, 11.582],
  ['Hamburg', 'Germany', 53.5511, 9.9937],
  ['Frankfurt', 'Germany', 50.1109, 8.6821],
  ['Zurich', 'Switzerland', 47.3769, 8.5417],
  ['Geneva', 'Switzerland', 46.2044, 6.1432],
  ['Vienna', 'Austria', 48.2082, 16.3738],
  ['Rome', 'Italy', 41.9028, 12.4964],
  ['Milan', 'Italy', 45.4642, 9.19],
  ['Venice', 'Italy', 45.4408, 12.3155],
  ['Athens', 'Greece', 37.9838, 23.7275],
  ['Prague', 'Czechia', 50.0755, 14.4378],
  ['Warsaw', 'Poland', 52.2297, 21.0122],
  ['Budapest', 'Hungary', 47.4979, 19.0402],
  ['Bucharest', 'Romania', 44.4268, 26.1025],
  ['Sofia', 'Bulgaria', 42.6977, 23.3219],
  ['Belgrade', 'Serbia', 44.7866, 20.4489],
  ['Copenhagen', 'Denmark', 55.6761, 12.5683],
  ['Stockholm', 'Sweden', 59.3293, 18.0686],
  ['Oslo', 'Norway', 59.9139, 10.7522],
  ['Helsinki', 'Finland', 60.1699, 24.9384],
  ['Tallinn', 'Estonia', 59.437, 24.7536],
  ['Riga', 'Latvia', 56.9496, 24.1052],
  ['Kyiv', 'Ukraine', 50.4501, 30.5234],
  ['Moscow', 'Russia', 55.7558, 37.6173],
  ['Saint Petersburg', 'Russia', 59.9311, 30.3609],
  ['Istanbul', 'Turkey', 41.0082, 28.9784],
  ['Ankara', 'Turkey', 39.9334, 32.8597],
  ['Cairo', 'Egypt', 30.0444, 31.2357],
  ['Alexandria', 'Egypt', 31.2001, 29.9187],
  ['Casablanca', 'Morocco', 33.5731, -7.5898],
  ['Marrakech', 'Morocco', 31.6295, -7.9811],
  ['Algiers', 'Algeria', 36.7538, 3.0588],
  ['Tunis', 'Tunisia', 36.8065, 10.1815],
  ['Dakar', 'Senegal', 14.7167, -17.4677],
  ['Accra', 'Ghana', 5.6037, -0.187],
  ['Lagos', 'Nigeria', 6.5244, 3.3792],
  ['Abuja', 'Nigeria', 9.0765, 7.3986],
  ['Addis Ababa', 'Ethiopia', 9.03, 38.74],
  ['Nairobi', 'Kenya', -1.2921, 36.8219],
  ['Kampala', 'Uganda', 0.3476, 32.5825],
  ['Dar es Salaam', 'Tanzania', -6.7924, 39.2083],
  ['Kinshasa', 'DR Congo', -4.4419, 15.2663],
  ['Luanda', 'Angola', -8.839, 13.2894],
  ['Johannesburg', 'South Africa', -26.2041, 28.0473],
  ['Durban', 'South Africa', -29.8587, 31.0218],
  ['Windhoek', 'Namibia', -22.5609, 17.0658],
  ['Maputo', 'Mozambique', -25.9692, 32.5732],
  ['Antananarivo', 'Madagascar', -18.8792, 47.5079],
  ['Dubai', 'United Arab Emirates', 25.2048, 55.2708],
  ['Abu Dhabi', 'United Arab Emirates', 24.4539, 54.3773],
  ['Riyadh', 'Saudi Arabia', 24.7136, 46.6753],
  ['Jeddah', 'Saudi Arabia', 21.4858, 39.1925],
  ['Doha', 'Qatar', 25.2854, 51.531],
  ['Tehran', 'Iran', 35.6892, 51.389],
  ['Baghdad', 'Iraq', 33.3152, 44.3661],
  ['Jerusalem', 'Israel', 31.7683, 35.2137],
  ['Tel Aviv', 'Israel', 32.0853, 34.7818],
  ['Amman', 'Jordan', 31.9454, 35.9284],
  ['Beirut', 'Lebanon', 33.8938, 35.5018],
  ['Tashkent', 'Uzbekistan', 41.2995, 69.2401],
  ['Almaty', 'Kazakhstan', 43.222, 76.8512],
  ['Karachi', 'Pakistan', 24.8607, 67.0011],
  ['Lahore', 'Pakistan', 31.5204, 74.3587],
  ['Islamabad', 'Pakistan', 33.6844, 73.0479],
  ['Delhi', 'India', 28.7041, 77.1025],
  ['New Delhi', 'India', 28.6139, 77.209],
  ['Kolkata', 'India', 22.5726, 88.3639],
  ['Chennai', 'India', 13.0827, 80.2707],
  ['Bangalore', 'India', 12.9716, 77.5946],
  ['Hyderabad', 'India', 17.385, 78.4867],
  ['Dhaka', 'Bangladesh', 23.8103, 90.4125],
  ['Kathmandu', 'Nepal', 27.7172, 85.324],
  ['Colombo', 'Sri Lanka', 6.9271, 79.8612],
  ['Bangkok', 'Thailand', 13.7563, 100.5018],
  ['Hanoi', 'Vietnam', 21.0285, 105.8542],
  ['Ho Chi Minh City', 'Vietnam', 10.8231, 106.6297],
  ['Phnom Penh', 'Cambodia', 11.5564, 104.9282],
  ['Kuala Lumpur', 'Malaysia', 3.139, 101.6869],
  ['Singapore', 'Singapore', 1.3521, 103.8198],
  ['Jakarta', 'Indonesia', -6.2088, 106.8456],
  ['Bali', 'Indonesia', -8.3405, 115.092],
  ['Manila', 'Philippines', 14.5995, 120.9842],
  ['Beijing', 'China', 39.9042, 116.4074],
  ['Shanghai', 'China', 31.2304, 121.4737],
  ['Shenzhen', 'China', 22.5431, 114.0579],
  ['Guangzhou', 'China', 23.1291, 113.2644],
  ['Chengdu', 'China', 30.5728, 104.0668],
  ['Hong Kong', 'China', 22.3193, 114.1694],
  ['Taipei', 'Taiwan', 25.033, 121.5654],
  ['Seoul', 'South Korea', 37.5665, 126.978],
  ['Busan', 'South Korea', 35.1796, 129.0756],
  ['Osaka', 'Japan', 34.6937, 135.5023],
  ['Kyoto', 'Japan', 35.0116, 135.7681],
  ['Sapporo', 'Japan', 43.0618, 141.3545],
  ['Ulaanbaatar', 'Mongolia', 47.8864, 106.9057],
  ['Vladivostok', 'Russia', 43.1155, 131.8855],
  ['Melbourne', 'Australia', -37.8136, 144.9631],
  ['Brisbane', 'Australia', -27.4698, 153.0251],
  ['Perth', 'Australia', -31.9505, 115.8605],
  ['Adelaide', 'Australia', -34.9285, 138.6007],
  ['Darwin', 'Australia', -12.4634, 130.8456],
  ['Canberra', 'Australia', -35.2809, 149.13],
  ['Auckland', 'New Zealand', -36.8485, 174.7633],
  ['Wellington', 'New Zealand', -41.2866, 174.7756],
  ['Suva', 'Fiji', -18.1416, 178.4419],
  ['Port Moresby', 'Papua New Guinea', -9.4438, 147.1803],
  ['Nuuk', 'Greenland', 64.1814, -51.6941],
];

const index = new Map();

for (const [name, country, lat, lng] of RAW) {
  if (!(Math.abs(lat) <= 90 && Math.abs(lng) <= 180)) {
    console.warn('gazetteer: dropping entry with invalid coordinates', name);
    continue;
  }
  const key = normalize(name);
  const entry = { name, country, lat, lng, countryKey: normalizeCountry(country) };
  if (!index.has(key)) index.set(key, []);
  index.get(key).push(entry);
}

export const GAZETTEER_SIZE = RAW.length;

/** Returns {lat, lng} for a record, or null when the city is unknown or the country conflicts. */
export function locate(record) {
  const name = normalize(record?.city);
  if (!name) return null;
  const candidates = index.get(NAME_ALIASES[name] ?? name);
  if (!candidates) return null;
  const country = normalizeCountry(record?.country);
  const match = country ? candidates.find((c) => c.countryKey === country) : candidates[0];
  return match ? { lat: match.lat, lng: match.lng } : null;
}
