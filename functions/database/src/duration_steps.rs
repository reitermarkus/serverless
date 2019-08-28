use chrono::{DateTime, Duration, Utc};

pub struct DurationSteps {
  begin: DateTime<Utc>,
  end: DateTime<Utc>,
  step: Duration,
}

impl Iterator for DurationSteps {
  type Item = (DateTime<Utc>, DateTime<Utc>);

  fn next(&mut self) -> Option<Self::Item> {
    if self.begin >= self.end {
      return None
    }

    let end = if let Some(end) = self.begin.checked_add_signed(self.step) {
      end
    } else {
      return None
    };

    let v = if end >= self.end {
      (self.begin.clone(), self.end.clone())
    } else {
      (self.begin.clone(), end)
    };

    self.begin = end;

    Some(v)
  }
}

pub trait IntoDurationSteps {
  fn into_duration_steps(self, steps: i32) -> DurationSteps;
}

impl IntoDurationSteps for (DateTime<Utc>, DateTime<Utc>) {
  fn into_duration_steps(self, steps: i32) -> DurationSteps {
    let (begin, end) = self;
    let duration = end.signed_duration_since(begin);
    let step = duration / steps;

    DurationSteps { begin, end, step }
  }
}
