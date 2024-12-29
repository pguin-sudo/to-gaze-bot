from aiogram.filters import BaseFilter
from aiogram.types import CallbackQuery, Message
from urlextract import URLExtract


class LinkFilter(BaseFilter):
    def __init__(self) -> None:
        self.urlextractor = URLExtract()

    async def __call__(self, obj: Message | CallbackQuery) -> bool:
        if isinstance(obj, Message):
            text = obj.text or obj.caption
            if not text:
                return False
            return self.urlextractor.has_urls(text)
        if isinstance(obj, CallbackQuery):
            text = obj.data
            if not text:
                return False
            return self.urlextractor.has_urls(text)
        return False
