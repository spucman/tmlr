use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct SpaceListResponse {
    pub data: Vec<SpaceResponse>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SpaceResponse {
    pub id: String,
    pub name: String,
    pub default: bool,
    pub members: Vec<SpaceMemberResponse>,
    pub retired_members: Vec<SpaceRetiredMemberResponse>,
}

#[derive(Deserialize, Debug)]
pub struct SpaceMemberResponse {
    pub id: String,
    pub name: String,
    pub email: String,
    pub role: String,
}

#[derive(Deserialize, Debug)]
pub struct SpaceRetiredMemberResponse {
    pub id: String,
    pub name: String,
}

impl SpaceListResponse {
    pub fn default_space(&self) -> Option<&SpaceResponse> {
        self.data.iter().find(|&x| x.default)
    }
}
