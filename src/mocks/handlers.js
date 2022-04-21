// すべてのリクエストハンドラを格納するモジュール
// src/mocks/handlers.js
import { rest } from 'msw'

export const handlers = [
	// req: 一致したリクエストに関する情報
	// res: モックレスポンスを作成するための機能ユーティリティ
	// ctx: 模擬応答のステータスコード、ヘッダ、ボディなどを設定するための関数群
	// rest.post('https://a-chat/api/login', (req, res, ctx) => {
	//   // Pユーザーの認証をセッションに永続させる
	//   sessionStorage.setItem('is-authenticated', 'true')
	//   return res(
	//     // 200のステータスコードで応答する
	//     ctx.status(200),
	//   )
	// }),
	// rest.get('https://a-chat/api/user', (req, res, ctx) => {
	//   // このセッションでユーザーが認証されているかどうかを確認する
	//   const isAuthenticated = sessionStorage.getItem('is-authenticated')
	//   if (!isAuthenticated) {
	//     // 認証されていない場合、403エラーで応答する
	//     return res(
	//       ctx.status(403),
	//       ctx.json({
	//         errorMessage: 'Not authorized',
	//       }),
	//     )
	//   }
	//   // 認証された場合、模擬ユーザの詳細を返します。
	//   return res(
	//     ctx.status(200),
	//     ctx.json({
	//       username: 'admin',
	//     }),
	//   )
	// }),
	// 会員登録
	rest.get('https://a-chat/api/signup', (req, res, ctx) => {
		// const parsedUrl = new URL(req.url)
		// const userId = parsedUrl.searchParams.get("userId")
		return res(
			ctx.status(200),
			ctx.json({
				"isAvailableUserId": true,
			}),
		)
	}),
	// ログイン認証
	rest.post('https://a-chat/api/login', (req, res, ctx) => {
		const { mail } = req.body
		const { password } = req.body
		return res(
			// 200のステータスコードで応答する
			ctx.status(200),
			ctx.json({
				"certificationResult": true
			})
		)
	}),
	// ニックネームまたはグループ名の検索でヒットするユーザーまたはグループ情報の取得
	rest.get('https://a-chat/api/home', (req, res, ctx) => {
		const parsedUrl = new URL(req.url)
		const search = parsedUrl.searchParams.get("search")
		return res(
			ctx.status(200),
			ctx.json(
				[
					{
						"friend": [
							{
								"direct_chat_room_id": "1",
								"friend_use_id": "asami111",
								"friend_profile_image": require("../../assets/images/friend_profile_image_1.jpg"),
								"friend_nickname": "検索結果name"
							}
						]
					},
					{
						"group": [
							{
								"group_chat_room_id": "12",
								"group_name": "検索結果グループ",
								"group_image": require("../../assets/images/group_image_1.jpg")
							}
						]
					}
				]
			),
		)
	}),
	// ユーザが所属するグループ一覧
	rest.get('https://a-chat/api/groups', (req, res, ctx) => {
		const parsedUrl = new URL(req.url)
		const userId = parsedUrl.searchParams.get("userId")
		return res(
			ctx.status(200),
			ctx.json(
				[
					{
						"group_chat_room_id": "1",
						"group_name": "group 1",
						"group_image": require("../../assets/images/group_image_1.jpg")
					},
					{
						"group_chat_room_id": "2",
						"group_name": "group 2",
						"group_image": require("../../assets/images/group_image_2.jpg")
					},
					{
						"group_chat_room_id": "3",
						"group_name": "group 3",
						"group_image": require("../../assets/images/group_image_2.jpg")
					},
					{
						"group_chat_room_id": "4",
						"group_name": "group 4",
						"group_image": require("../../assets/images/group_image_2.jpg")
					},
				]
			),
		)
	}),
	// ユーザーの所属するグループ数
	rest.get('https://a-chat/api/group-count', (req, res, ctx) => {
		const parsedUrl = new URL(req.url)
		const userId = parsedUrl.searchParams.get("userId")
		return res(
			ctx.status(200),
			ctx.text(1),
		)
	}),
	// ユーザの友達数
	rest.get('https://a-chat/api/friend-count', (req, res, ctx) => {
		const parsedUrl = new URL(req.url)
		const userId = parsedUrl.searchParams.get("userId")
		return res(
			ctx.status(200),
			ctx.text(1),
		)
	}),
	// ユーザーの友達一覧
	rest.get('https://a-chat/api/friends', (req, res, ctx) => {
		const parsedUrl = new URL(req.url)
		const userId = parsedUrl.searchParams.get("userId")
		return res(
			ctx.status(200),
			ctx.json(
				[
					{
						"direct_chat_room_id": "1",
						"friend_use_id": "friend 1",
						"friend_profile_image": require("../../assets/images/friend_profile_image_1.jpg"),
						"friend_nickname": "asamiasamiasami1"
					},
					{
						"direct_chat_room_id": "2",
						"friend_use_id": "friend 2",
						"friend_profile_image": require("../../assets/images/friend_profile_image_2.jpg"),
						"friend_nickname": "asami2"
					},
					{
						"direct_chat_room_id": "3",
						"friend_use_id": "friend 3",
						"friend_profile_image": require("../../assets/images/friend_profile_image_2.jpg"),
						"friend_nickname": "asami3"
					},
					{
						"direct_chat_room_id": "5",
						"friend_use_id": "friend 5",
						"friend_profile_image": require("../../assets/images/friend_profile_image_2.jpg"),
						"friend_nickname": "asami5"
					},
					{
						"direct_chat_room_id": "6",
						"friend_use_id": "friend 6",
						"friend_profile_image": require("../../assets/images/friend_profile_image_2.jpg"),
						"friend_nickname": "asami6"
					},
					{
						"direct_chat_room_id": "7",
						"friend_use_id": "friend 7",
						"friend_profile_image": require("../../assets/images/friend_profile_image_2.jpg"),
						"friend_nickname": "asami7"
					},
					{
						"direct_chat_room_id": "8",
						"friend_use_id": "friend 8",
						"friend_profile_image": require("../../assets/images/friend_profile_image_2.jpg"),
						"friend_nickname": "asami8"
					},
					{
						"direct_chat_room_id": "9",
						"friend_use_id": "friend 9",
						"friend_profile_image": require("../../assets/images/friend_profile_image_2.jpg"),
						"friend_nickname": "asami9"
					},
					{
						"direct_chat_room_id": "4",
						"friend_use_id": "friend 4",
						"friend_profile_image": require("../../assets/images/friend_profile_image_2.jpg"),
						"friend_nickname": "asami4"
					},
					{
						"direct_chat_room_id": "10",
						"friend_use_id": "friend 10",
						"friend_profile_image": require("../../assets/images/friend_profile_image_2.jpg"),
						"friend_nickname": "asami10"
					},
				]
			),
		)
	}),
]