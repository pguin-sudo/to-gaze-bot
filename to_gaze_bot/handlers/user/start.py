from aiogram import html, types
from aiogram.fsm.context import FSMContext

from to_gaze_bot import states


async def start(msg: types.Message, state: FSMContext) -> None:
    if msg.from_user is None:
        return
    m = [
        f'Привет, <a href="tg://user?id={msg.from_user.id}">{html.quote(msg.from_user.full_name)}</a>',
        "Просто скинь мне любую ссылку на видео и я скачаю его",
    ]
    await msg.answer("\n".join(m))
    await state.set_state(states.user.UserMainMenu.menu)
