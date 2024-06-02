use std::fs::File;
use std::io;
use std::io::{Read, Write};
use std::path::Path;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct HighScore {
    score: i32,
    timestamp: DateTime<Utc>,
}

pub struct HighScoreManager {
    file_path: String,
}

impl HighScoreManager {
    pub fn new(file_path: &str) -> Self {
        Self {
            file_path: file_path.to_string(),
        }
    }

    fn read_highscore(&self) -> io::Result<HighScore> {
        if !Path::new(&self.file_path).exists() {
            return Ok(HighScore {
                score: 0,
                timestamp: Utc::now(),
            });
        }

        let mut file = File::open(&self.file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let highscore: HighScore = serde_json::from_str(&contents)
            .unwrap_or(HighScore {
                score: 0,
                timestamp: Utc::now(),
            });

        Ok(highscore)
    }

    fn write_highscore(&self, highscore: &HighScore) -> io::Result<()> {
        let serialized = serde_json::to_string(highscore)?;
        let mut file = File::create(&self.file_path)?;
        file.write_all(serialized.as_bytes())
    }

    pub fn update_highscore(&self, new_score: i32) -> io::Result<ScoreInfo> {
        let current_highscore = self.read_highscore()?;
        if new_score > current_highscore.score {
            let new_highscore = HighScore {
                score: new_score,
                timestamp: Utc::now(),
            };
            self.write_highscore(&new_highscore)?;
            Ok(ScoreInfo::NewRecord)
        } else {
            Ok(ScoreInfo::HistoryRecord(current_highscore.score))
        }
    }
}

pub enum ScoreInfo {
    NewRecord,
    HistoryRecord(i32),
}