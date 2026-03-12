pub trait BuilderTrait {
    type Target;
    type Err;
    async fn build(self) -> Result<Self::Target, Self::Err>;
}
