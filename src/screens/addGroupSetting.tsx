// libs
import React, { useState, useEffect } from 'react';
import { View, SafeAreaView, KeyboardAvoidingView } from 'react-native';
import { API_SERVER_URL } from "../constants/api"

// components
import { AddGroupTitle } from '../components/addGroup/addGroupTitle'
import { TopAreaWrapper } from "../components/common/topAreaWrapper"
import { GroupImageAndGroupName } from "../components/common/_topAreaContainer/groupImageAndGroupName"
import { SmallButton } from "../components/common/smallButton"
import { SelectedFriendSpace } from "../components/addGroupSetting/selectedFriendSpace"

// sameStyles
import { sameStyles } from '../constants/styles/sameStyles'

// layouts
import { IPHONE_X_BOTTOM_SPACE } from '../constants/layout'

export function AddGroupSetting({ route, navigation }) {
	// ユーザーID(今後は認証から取得するようにする)
	const userId = "asami11"
	const [friendList, setFriendList] = useState(route.params.friendList)
	const groupMemberCount = friendList.length + 1 // 自分を1としてカウントし、足す

	// 自分のニックネーム
	const [ownNickName, setOwnNickName] = useState('')
	// 自分のプロフィール画像
	const [ownProfileImage, setOwnProfileImage] = useState('')

	// グループ画像
	const [image, setImage] = useState(null)

	// グループ名のplaceholderを生成
	let friendListNames = ''
	if (ownNickName && friendList) {
		// 一番最初に選んだメンバーの名前を取得
		friendListNames = `${ownNickName}`
		// 選択された友達リストからニックネームだけを取り出す
		for (let i = 0; i < friendList.length; i++) {
			friendListNames = friendListNames + ', ' + friendList[i].friend_nickname
		}
	}

	// グループ名
	const [groupName, setGroupName] = useState(friendListNames)

	// [自分の情報]ユーザーIDに紐づくニックネーム、プロフィール画像の取得
	async function _fetchProfileByUserId(userId) {
		try {
			// APIリクエスト
			const response = await fetch(API_SERVER_URL + `/api/users/${userId}/profile`, {
				method: "GET",
				headers: {
					"Content-Type": "application/json"
				},
			})
			// レスポンスをJSONにする
			const parse_response = await response.json()
			// 自分のニックネームの設定
			if (parse_response.nickName) {
				setOwnNickName(parse_response.nickName)
			}
			if (parse_response.profileImage) {
				setOwnProfileImage(parse_response.profileImage)
			}
		} catch (e) {
			console.error(e)
		}
	}

	useEffect(() => {
		if (userId) {
			_fetchProfileByUserId(userId)
		}
	}, [])

	return (
		<KeyboardAvoidingView behavior="padding" style={sameStyles.screenContainerStyle}>
			<SafeAreaView style={sameStyles.screenContainerStyle}>
				{/* 画面一番上にある青色の余白部分 */}
				<View style={sameStyles.topMarginViewStyle}></View>
				{/* 丸みを帯びている白いトップ部分 */}
				<TopAreaWrapper type={"addGroupSetting"}>
					<GroupImageAndGroupName image={image} setImage={setImage} groupName={groupName} setGroupName={setGroupName} friendListNames={friendListNames} />
				</TopAreaWrapper>
				{/* トップ部分を除くメイン部分: iphoneXの場合は、底のマージンを考慮 */}
				<View style={IPHONE_X_BOTTOM_SPACE === 0 ? sameStyles.withFooterMainContainerStyle : sameStyles.withFooterMainContainerIphoneXStyle}>
					{/* タイトル */}
					<AddGroupTitle text={"Member"} groupMemberCount={groupMemberCount} />
					{/* 選択された友達のスペース */}
					<SelectedFriendSpace navigation={navigation} friendList={friendList} setFriendList={setFriendList} ownNickName={ownNickName} ownProfileImage={ownProfileImage} />
				</View>
				{/* 右下のボタン(Create) */}
				{friendListNames.length !== 0 && (
					<SmallButton text={"Create"} navigation={navigation} friendList={friendList} groupSetting={{ "groupName": groupName, "image": image }} type={"addGroupSetting"} friendListNames={friendListNames} alreadyFriend={null} addGroupMemberGroupChatRoomId={null} addGroupMemberGroupImage={null} addGroupMemberGroupName={null} />
				)}
			</SafeAreaView>
		</KeyboardAvoidingView>
	);
}
