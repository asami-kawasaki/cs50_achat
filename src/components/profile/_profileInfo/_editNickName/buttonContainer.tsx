// libs
import React from 'react';
import { View, StyleSheet, Text } from 'react-native';
import { API_SERVER_URL } from "../../../../constants/api"

// components
import { Button } from "../../../common/button"

// layouts
import { STANDARD_FONT, MAIN_PINK_COLOR } from '../../../../constants/layout'

export function ButtonContainer({ navigation, nickName, setNickName, wordCount, isValidInput, defaultInput }) {

	// ユーザーID(今後は認証から取得するようにする)
	const userId = "asami11"

	// ニックネームの更新
	async function _updateNickName() {
		try {
			// APIリクエスト
			const bodyData = {
				"nickName": nickName,
			}
			const response = await fetch(API_SERVER_URL + `/api/users/${userId}/profile`, {
				method: "POST",
				headers: {
					"Content-Type": "application/json"
				},
				body: JSON.stringify(bodyData),
			})
		} catch (e) {
			console.error(e)
		}
	}

	return (
		<View style={styles.buttonContainerStyle}>
			{isValidInput && !defaultInput && wordCount >= 1 && (
				<Button navigation={navigation} link={"Profile"} buttonText={'Save'} enable={true} scene={'ProfileSettingNickName'} propsList={{ "_updateNickName": _updateNickName, "nickName": nickName, "setNickName": setNickName }} />
			)}
			{(!isValidInput && !defaultInput) && (
				<View style={styles.inValidErrorContainerStyle}>
					{!defaultInput && (
						<Text style={styles.errorText}>Nicknames can be entered from a single character.</Text>
					)}
					<Button navigation={navigation} link={null} buttonText={'Save'} enable={false} scene={'ProfileSettingNickName'} propsList={null} />
				</View>
			)}
		</View>
	);
}


const styles = StyleSheet.create({
	buttonContainerStyle: {
		marginTop: 32,
		justifyContent: "center",
		alignItems: "center",
	},
	inValidErrorContainerStyle: {
		justifyContent: "center",
		alignItems: "center",
	},
	errorText: {
		fontFamily: STANDARD_FONT,
		color: MAIN_PINK_COLOR,
		marginBottom: 32,
	}
})
