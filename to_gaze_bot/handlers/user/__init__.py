from aiogram import Router
from aiogram.filters import CommandStart, StateFilter

from to_gaze_bot import states
from to_gaze_bot.filters import ChatTypeFilter, TextFilter, LinkFilter

from . import start, video


def prepare_router() -> Router:
    user_router = Router()
    user_router.message.filter(ChatTypeFilter("private"))

    user_router.message.register(start.start, CommandStart())
    user_router.message.register(
        start.start,
        TextFilter("🏠 Главное меню"),  # noqa: RUF001
        StateFilter(states.user.UserMainMenu.menu),
    )

    user_router.message.register(
        video.video,
        LinkFilter(),  # noqa: RUF001
        StateFilter(states.user.UserMainMenu.menu),
    )

    return user_router
