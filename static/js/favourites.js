import { normalize } from './gazetteer.js';

/** Favourites are matched to globe cities by normalized city name only. */
export function favouriteKeyOf(name) {
  return normalize(String(name ?? '').trim());
}

export function isFavourite(favourites, city) {
  const key = favouriteKeyOf(city?.record?.city ?? city?.name);
  return !!key && favourites.some((f) => favouriteKeyOf(f.city) === key);
}

/** Pairs each favourite with its loaded city (or null) in server order. */
export function favouriteItems(favourites, cities) {
  return favourites.map((favourite) => {
    const key = favouriteKeyOf(favourite.city);
    const city = cities.find((c) => favouriteKeyOf(c.record?.city ?? c.name) === key) ?? null;
    return { favourite, city };
  });
}
