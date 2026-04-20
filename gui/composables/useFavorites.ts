const favorites = useState<string[]>('favorites', () => [])

export const useFavorites = () => {
  onMounted(() => {
    try {
      const stored = localStorage.getItem('passcli-favorites')
      if (stored) favorites.value = JSON.parse(stored)
    } catch {}
  })

  const isFavorite = (service: string) => favorites.value.includes(service)

  const toggleFavorite = (service: string) => {
    if (isFavorite(service)) {
      favorites.value = favorites.value.filter(s => s !== service)
    } else {
      favorites.value = [...favorites.value, service]
    }
    localStorage.setItem('passcli-favorites', JSON.stringify(favorites.value))
  }

  return { isFavorite, toggleFavorite }
}
