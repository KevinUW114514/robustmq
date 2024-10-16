// Copyright 2023 RobustMQ Team
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#[derive(Debug, Clone)]
pub struct Handler {}

impl Handler {
    pub fn new() -> Handler {
        Handler {}
    }

    pub async fn write(&self) {}

    pub async fn read(&self) {}

    pub async fn active_segment(&self) {}

    pub async fn offset_commit(&self) {}
}